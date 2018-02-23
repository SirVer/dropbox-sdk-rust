from contextlib import nested

from rust import RustHelperBackend
from stone import ir
from stone.backends.helpers import split_words


def fmt_shouting_snake(name):
    return '_'.join([word.upper() for word in split_words(name)])


class RustBackend(RustHelperBackend):
    def __init__(self, target_folder_path, args):
        super(RustBackend, self).__init__(target_folder_path, args)
        self._modules = []
        self.preserve_aliases = True

    # File Generators

    def generate(self, api):
        for namespace in api.namespaces.values():
            self._emit_namespace(namespace)
        self._generate_mod_file()

    def _generate_mod_file(self):
        with self.output_to_relative_path('mod.rs'):
            self._emit_header()
            for module in self._modules:
                self.emit(u'#[cfg(feature = "dbx_{}")]'.format(module))
                self.emit(u'pub mod {};'.format(module))
                self.emit()

    # Type Emitters

    def _emit_namespace(self, namespace):
        with self.output_to_relative_path(namespace.name + '.rs'):
            self._current_namespace = namespace.name
            self._emit_header()

            if namespace.doc is not None:
                self.emit_wrapped_text(namespace.doc, prefix=u'//! ', width=100)
                self.emit()

            for alias in namespace.aliases:
                self._emit_alias(alias)
            if namespace.aliases:
                self.emit()

            for fn in namespace.routes:
                self._emit_route(namespace.name, fn)

            for typ in namespace.data_types:
                if isinstance(typ, ir.Struct):
                    if typ.has_enumerated_subtypes():
                        self._emit_polymorphic_struct(typ)
                    else:
                        self._emit_struct(typ)
                elif isinstance(typ, ir.Union):
                    self._emit_union(typ)
                else:
                    raise RuntimeError('WARNING: unhandled type "{}" of field "{}"'
                                       .format(type(typ).__name__, typ.name))

        self._modules.append(namespace.name)

    def _emit_header(self):
        self.emit(u'// DO NOT EDIT')
        self.emit(u'// This file was generated by Stone')
        self.emit()
        self.emit(u'#![allow(')
        self.emit(u'    unknown_lints,  // keep rustc from complaining about clippy lints')
        self.emit(u'    too_many_arguments,')
        self.emit(u'    large_enum_variant,')
        self.emit(u'    doc_markdown,')
        self.emit(u')]')
        self.emit()

    def _emit_struct(self, struct):
        struct_name = self.struct_name(struct)
        self._emit_doc(struct.doc)
        self.emit(u'#[derive(Debug)]')
        with self.block(u'pub struct {}'.format(struct_name)):
            for field in struct.all_fields:
                self._emit_doc(field.doc)
                self.emit(u'pub {}: {},'.format(
                    self.field_name(field),
                    self._rust_type(field.data_type)))
        self.emit()

        if not struct.all_required_fields:
            self._impl_default_for_struct(struct)
            self.emit()

        if struct.all_required_fields:
            with self._impl_struct(struct):
                if struct.all_required_fields:
                    self._emit_new_for_struct(struct)
                self.emit()
            self.emit()

        self._impl_serde_for_struct(struct)

    def _emit_polymorphic_struct(self, struct):
        enum_name = self.enum_name(struct)
        self._emit_doc(struct.doc)
        self.emit(u'#[derive(Debug)]')
        with self.block(u'pub enum {}'.format(enum_name)):
            for subtype in struct.get_enumerated_subtypes():
                self.emit(u'{}({}),'.format(
                    self.enum_variant_name(subtype),
                    self._rust_type(subtype.data_type)))
            if struct.is_catch_all():
                self.emit(u'_Unknown')
        self.emit()

        self._impl_serde_for_polymorphic_struct(struct)

    def _emit_union(self, union):
        enum_name = self.enum_name(union)
        self._emit_doc(union.doc)
        self.emit(u'#[derive(Debug)]')
        with self.block(u'pub enum {}'.format(enum_name)):
            for field in union.all_fields:
                self._emit_doc(field.doc)
                variant_name = self.enum_variant_name(field)
                if isinstance(field.data_type, ir.Void):
                    self.emit(u'{},'.format(variant_name))
                else:
                    self.emit(u'{}({}),'.format(variant_name, self._rust_type(field.data_type)))
        self.emit()

        self._impl_serde_for_union(union)

        if union.name.endswith('Error'):
            self._impl_error(enum_name)

    def _emit_route(self, ns, fn):
        route_name = self.route_name(fn)
        self._emit_doc(fn.doc)
        host = fn.attrs.get('host', 'api')
        if host == 'api':
            endpoint = u'::client_trait::Endpoint::Api'
        elif host == 'content':
            endpoint = u'::client_trait::Endpoint::Content'
        elif host == 'notify':
            endpoint = u'::client_trait::Endpoint::Notify'
        else:
            raise RuntimeError(u'ERROR: unsupported endpoint: {}'.format(host))

        style = fn.attrs.get('style', 'rpc')
        if style == 'rpc':
            with self.emit_rust_function_def(
                    route_name,
                    [u'client: &::client_trait::HttpClient',
                        u'arg: &{}'.format(self._rust_type(fn.arg_data_type))],
                    u'::Result<Result<{}, {}>>'.format(
                        self._rust_type(fn.result_data_type),
                        self._rust_type(fn.error_data_type)),
                    access=u'pub'):
                self.emit_rust_fn_call(
                    u'::client_helpers::request',
                    [u'client',
                        endpoint,
                        u'"{}/{}"'.format(ns, fn.name),
                        u'arg',
                        u'None'])
        elif style == 'download':
            with self.emit_rust_function_def(
                    route_name,
                    [u'client: &::client_trait::HttpClient',
                        u'arg: &{}'.format(self._rust_type(fn.arg_data_type)),
                        u'range_start: Option<u64>',
                        u'range_end: Option<u64>'],
                    u'::Result<Result<::client_trait::HttpRequestResult<{}>, {}>>'.format(
                        self._rust_type(fn.result_data_type),
                        self._rust_type(fn.error_data_type)),
                    access=u'pub'):
                self.emit_rust_fn_call(
                    u'::client_helpers::request_with_body',
                    [u'client',
                        endpoint,
                        u'"{}/{}"'.format(ns, fn.name),
                        u'arg',
                        u'None',
                        u'range_start',
                        u'range_end'])
        elif style == 'upload':
            with self.emit_rust_function_def(
                    route_name,
                    [u'client: &::client_trait::HttpClient',
                        u'arg: &{}'.format(self._rust_type(fn.arg_data_type)),
                        u'body: Vec<u8>'],
                    u'::Result<Result<::client_trait::HttpRequestResult<{}>, {}>>'.format(
                        self._rust_type(fn.result_data_type),
                        self._rust_type(fn.error_data_type)),
                    access=u'pub'):
                self.emit_rust_fn_call(
                    u'::client_helpers::request_with_body',
                    [u'client',
                        endpoint,
                        u'"{}/{}"'.format(ns, fn.name),
                        u'arg',
                        u'Some(body)',
                        u'None',
                        u'None'])
        else:
            raise RuntimeError(u'ERROR: unknown route style: {}'.format(style))
        self.emit()

    def _emit_alias(self, alias):
        alias_name = self.alias_name(alias)
        self.emit(u'pub type {} = {};'.format(alias_name, self._rust_type(alias.data_type)))

    # Serialization

    def _impl_serde_for_struct(self, struct):
        type_name = self.struct_name(struct)
        field_list_name = u'{}_FIELDS'.format(fmt_shouting_snake(struct.name))
        self.generate_multiline_list(
            list(u'"{}"'.format(field.name) for field in struct.all_fields),
            before='const {}: &[&str] = &'.format(field_list_name),
            after=';',
            delim=(u'[', u']'))
        with self._impl_struct(struct):
            with self.emit_rust_function_def(
                    u'internal_deserialize<\'de, V: ::serde::de::MapAccess<\'de>>',
                    [u'mut map: V'],
                    u'Result<{}, V::Error>'.format(type_name),
                    access=u'pub(crate)'):
                self.emit(u'use serde::de;')
                if not struct.all_fields:
                    with self.block(u'if let Some(key) = map.next_key()?'):
                        self.emit(u'return Err(de::Error::unknown_field(key, {}));'
                                  .format(field_list_name))
                else:
                    for field in struct.all_fields:
                        self.emit(u'let mut field_{} = None;'.format(self.field_name(field)))
                    with nested(self.block(u'while let Some(key) = map.next_key()?'),
                                self.block(u'match key')):
                        for field in struct.all_fields:
                            field_name = self.field_name(field)
                            with self.block(u'"{}" =>'.format(field.name)):
                                with self.block(u'if field_{}.is_some()'.format(field_name)):
                                    self.emit(u'return Err(de::Error::duplicate_field("{}"));'
                                              .format(field.name))
                                self.emit(u'field_{} = Some(map.next_value()?);'.format(field_name))
                        self.emit(u'_ => return Err(de::Error::unknown_field(key, {}))'
                                  .format(field_list_name))
                with self.block(u'Ok({}'.format(type_name), delim=(u'{', u'})')):
                    for field in struct.all_fields:
                        field_name = self.field_name(field)
                        if isinstance(field.data_type, ir.Nullable):
                            self.emit(u'{}: field_{},'.format(field_name, field_name))
                        elif field.has_default:
                            if isinstance(field.data_type, ir.String) \
                                    and not field.default:
                                self.emit(u'{}: field_{}.unwrap_or_else(String::new),'
                                          .format(field_name, field_name))
                            elif ir.is_primitive_type(ir.unwrap_aliases(field.data_type)[0]):
                                self.emit(u'{}: field_{}.unwrap_or({}),'
                                          .format(field_name,
                                                  field_name,
                                                  self._default_value(field)))
                            else:
                                self.emit(u'{}: field_{}.unwrap_or_else(|| {}),'
                                          .format(field_name,
                                                  field_name,
                                                  self._default_value(field)))
                        else:
                            self.emit(u'{}: field_{}.ok_or_else(|| de::Error::missing_field("{}"))?,'
                                      .format(field_name, field_name, field.name))
            if struct.all_fields:
                self.emit()
                with self.emit_rust_function_def(
                        u'internal_serialize<S: ::serde::ser::Serializer>',
                        [u'&self', u's: &mut S::SerializeStruct'],
                        u'Result<(), S::Error>',
                        access=u'pub(crate)'):
                    self.emit(u'use serde::ser::SerializeStruct;')
                    self.generate_multiline_list(
                        list(u's.serialize_field("{}", &self.{})'
                             .format(field.name, self.field_name(field))
                             for field in struct.all_fields),
                        delim=(u'', u''),
                        sep='?;',
                        skip_last_sep=True)
        self.emit()
        with self._impl_deserialize(self.struct_name(struct)):
            self.emit(u'// struct deserializer')
            self.emit(u'use serde::de::{MapAccess, Visitor};')
            self.emit(u'struct StructVisitor;')
            with self.block(u'impl<\'de> Visitor<\'de> for StructVisitor'):
                self.emit(u'type Value = {};'.format(type_name))
                with self.emit_rust_function_def(
                        u'expecting',
                        [u'&self', u'f: &mut ::std::fmt::Formatter'],
                        u'::std::fmt::Result'):
                    self.emit(u'f.write_str("a {} struct")'.format(struct.name))
                with self.emit_rust_function_def(
                        u'visit_map<V: MapAccess<\'de>>',
                        [u'self', u'map: V'],
                        u'Result<Self::Value, V::Error>'):
                    self.emit(u'{}::internal_deserialize(map)'.format(type_name))
            self.emit(u'deserializer.deserialize_struct("{}", {}, StructVisitor)'
                      .format(struct.name,
                              field_list_name))
        self.emit()
        with self._impl_serialize(type_name):
            self.emit(u'// struct serializer')
            self.emit(u'use serde::ser::SerializeStruct;')
            if not struct.all_fields:
                self.emit(u'serializer.serialize_struct("{}", 0)?.end()'.format(struct.name))
            else:
                self.emit(u'let mut s = serializer.serialize_struct("{}", {})?;'
                          .format(struct.name,
                                  len(struct.all_fields)))
                self.emit(u'self.internal_serialize::<S>(&mut s)?;')
                self.emit(u's.end()')
        self.emit()

    def _impl_serde_for_polymorphic_struct(self, struct):
        type_name = self.enum_name(struct)
        with self._impl_deserialize(type_name):
            self.emit(u'// polymorphic struct deserializer')
            self.emit(u'use serde::de::{self, MapAccess, Visitor};')
            self.emit(u'struct EnumVisitor;')
            with self.block(u'impl<\'de> Visitor<\'de> for EnumVisitor'):
                self.emit(u'type Value = {};'.format(type_name))
                with self.emit_rust_function_def(
                        u'expecting',
                        [u'&self', u'f: &mut ::std::fmt::Formatter'],
                        u'::std::fmt::Result'):
                    self.emit(u'f.write_str("a {} structure")'.format(struct.name))
                with self.emit_rust_function_def(
                        u'visit_map<V: MapAccess<\'de>>',
                        [u'self', u'mut map: V'],
                        u'Result<Self::Value, V::Error>'):
                    with self.block(u'let tag = match map.next_key()?', after=';'):
                        self.emit(u'Some(".tag") => map.next_value()?,')
                        self.emit(u'_ => return Err(de::Error::missing_field(".tag"))')
                    with self.block(u'match tag'):
                        for subtype in struct.get_enumerated_subtypes():
                            variant_name = self.enum_variant_name(subtype)
                            if isinstance(subtype.data_type, ir.Void):
                                self.emit(u'"{}" => Ok({}::{}),'
                                          .format(subtype.name, type_name, variant_name))
                            elif isinstance(ir.unwrap_aliases(subtype.data_type)[0], ir.Struct) \
                                    and not subtype.data_type.has_enumerated_subtypes():
                                self.emit(u'"{}" => Ok({}::{}({}::internal_deserialize(map)?)),'
                                          .format(subtype.name,
                                                  type_name,
                                                  variant_name,
                                                  self._rust_type(subtype.data_type)))
                            else:
                                with self.block(u'"{}" =>'.format(subtype.name)):
                                    with self.block(u'if map.next_key()? != Some("{}")'
                                                    .format(subtype.name)):
                                        self.emit(u'Err(de::Error::missing_field("{}"));'
                                                  .format(subtype.name))
                                    self.emit(u'Ok({}::{}(map.next_value()?))'
                                              .format(type_name, variant_name))
                        if struct.is_catch_all():
                            self.emit(u'_ => Ok({}::_Unknown)'.format(type_name))
                        else:
                            self.emit(u'_ => Err(de::Error::unknown_variant(tag, VARIANTS))')
            self.generate_multiline_list(
                list(u'"{}"'.format(subtype.name)
                     for field in struct.get_enumerated_subtypes()),
                before='const VARIANTS: &[&str] = &',
                after=';',
                delim=(u'[', u']'))
            self.emit(u'deserializer.deserialize_struct("{}", VARIANTS, EnumVisitor)'.format(
                struct.name))
        self.emit()
        with self._impl_serialize(type_name):
            self.emit(u'// polymorphic struct serializer')
            self.emit(u'use serde::ser::SerializeStruct;')
            with self.block(u'match *self'):
                for subtype in struct.get_enumerated_subtypes():
                    variant_name = self.enum_variant_name(subtype)
                    with self.block(u'{}::{}(ref x) =>'.format(type_name, variant_name)):
                        self.emit(u'let mut s = serializer.serialize_struct("{}", {})?;'
                                  .format(type_name, len(subtype.data_type.all_fields) + 1))
                        self.emit(u's.serialize_field(".tag", "{}")?;'.format(subtype.name))
                        for field in subtype.data_type.all_fields:
                            self.emit(u's.serialize_field("{}", &x.{})?;'
                                      .format(field.name,
                                              self.field_name(field)))
                        self.emit(u's.end()')
                if struct.is_catch_all():
                    self.emit(u'{}::_Unknown => Err(::serde::ser::Error::custom("cannot serialize '
                              u'unknown variant"))'.format(
                                    type_name))
        self.emit()

    def _impl_serde_for_union(self, union):
        type_name = self.enum_name(union)
        with self._impl_deserialize(type_name):
            self.emit(u'// union deserializer')
            self.emit(u'use serde::de::{self, MapAccess, Visitor};')
            self.emit(u'struct EnumVisitor;')
            with self.block(u'impl<\'de> Visitor<\'de> for EnumVisitor'):
                self.emit(u'type Value = {};'.format(type_name))
                with self.emit_rust_function_def(
                        u'expecting',
                        [u'&self', u'f: &mut ::std::fmt::Formatter'],
                        u'::std::fmt::Result'):
                    self.emit(u'f.write_str("a {} structure")'.format(union.name))
                with self.emit_rust_function_def(
                        u'visit_map<V: MapAccess<\'de>>',
                        [u'self', u'mut map: V'],
                        u'Result<Self::Value, V::Error>'):
                    with self.block(u'let tag: &str = match map.next_key()?', after=';'):
                        self.emit(u'Some(".tag") => map.next_value()?,')
                        self.emit(u'_ => return Err(de::Error::missing_field(".tag"))')
                    with self.block(u'match tag'):
                        for field in union.all_fields:
                            if field.catch_all:
                                # Handle the 'Other' variant at the end.
                                continue
                            variant_name = self.enum_variant_name(field)
                            ultimate_type = ir.unwrap(field.data_type)[0]
                            if isinstance(field.data_type, ir.Void):
                                self.emit(u'"{}" => Ok({}::{}),'
                                          .format(field.name, type_name, variant_name))
                            elif isinstance(ultimate_type, ir.Struct) \
                                    and not ultimate_type.has_enumerated_subtypes():
                                if isinstance(ir.unwrap_aliases(field.data_type)[0], ir.Nullable):
                                    with self.block(u'"{}" =>'.format(field.name)):
                                        # HACK ALERT
                                        # A nullable here means we might have more fields that can
                                        # be deserialized into the inner type, or we might have
                                        # nothing, meaning None. Serde maps don't have a peek
                                        # method, so instead we have this hack. Unfortunately, in
                                        # the case where size_hint returns None, we have to just try
                                        # and see what happens, and errors will be silently
                                        # interpreted as the inner data being None.
                                        with self.block(u'match map.size_hint()'):
                                            self.emit(u'Some(0) => Ok({}::{}(None)),'
                                                      .format(type_name, variant_name))
                                            self.emit(u'Some(_) => Ok({}::{}(Some('
                                                      u'{}::internal_deserialize(map)?))),'
                                                      .format(type_name,
                                                              variant_name,
                                                              self._rust_type(ultimate_type)))
                                            with self.block(u'None => '
                                                            'match {}::internal_deserialize(map)'
                                                            .format(self._rust_type(
                                                                ultimate_type))):
                                                self.emit(u'Ok(inner) => Ok({}::{}(Some(inner))),'
                                                          .format(type_name, variant_name))
                                                # silently ignore error and treat it as None :(
                                                self.emit(u'Err(_) => Ok({}::{}(None))'
                                                          .format(type_name, variant_name))
                                else:
                                    self.emit(u'"{}" => Ok({}::{}({}::internal_deserialize(map)?)),'
                                              .format(field.name,
                                                      type_name,
                                                      variant_name,
                                                      self._rust_type(field.data_type)))
                            else:
                                with self.block(u'"{}" =>'.format(field.name)):
                                    with self.block(u'match map.next_key()?'):
                                        self.emit(u'Some("{}") => Ok({}::{}(map.next_value()?)),'
                                                  .format(field.name,
                                                          type_name,
                                                          variant_name))
                                        if isinstance(ir.unwrap_aliases(field.data_type)[0],
                                                      ir.Nullable):
                                            # if it's null, the field can be omitted entirely
                                            self.emit(u'None => Ok({}::{}(None)),'
                                                      .format(type_name, variant_name))
                                        else:
                                            self.emit(u'None => Err(de::Error::missing_field("{}")),'
                                                      .format(field.name))
                                        self.emit(u'_ => Err(de::Error::unknown_field(tag, VARIANTS))')
                        if not union.closed:
                            self.emit(u'_ => Ok({}::Other)'.format(type_name))
                        else:
                            self.emit(u'_ => Err(de::Error::unknown_variant(tag, VARIANTS))')
            self.generate_multiline_list(
                    list(u'"{}"'.format(field.name) for field in union.all_fields),
                    before='const VARIANTS: &[&str] = &',
                    after=';',
                    delim=(u'[', u']'),)
            self.emit(u'deserializer.deserialize_struct("{}", VARIANTS, EnumVisitor)'.format(
                union.name))
        self.emit()
        with self._impl_serialize(type_name):
            self.emit(u'// union serializer')
            if len(union.all_fields) == 1 and union.all_fields[0].catch_all:
                # special case: an open union with no variants defined.
                self.emit(u'#![allow(unused_variables)]')
                self.emit(u'Err(::serde::ser::Error::custom("cannot serialize an open union with '
                          u'no defined variants"))')
            else:
                self.emit(u'use serde::ser::SerializeStruct;')
                with self.block(u'match *self'):
                    for field in union.all_fields:
                        if field.catch_all:
                            # Handle the 'Other' variant at the end.
                            continue
                        variant_name = self.enum_variant_name(field)
                        if isinstance(field.data_type, ir.Void):
                            with self.block(u'{}::{} =>'.format(type_name, variant_name)):
                                self.emit(u'// unit')
                                self.emit(u'let mut s = serializer.serialize_struct("{}", 1)?;'
                                          .format(union.name))
                                self.emit(u's.serialize_field(".tag", "{}")?;'.format(field.name))
                                self.emit(u's.end()')
                        else:
                            ultimate_type = ir.unwrap(field.data_type)[0]
                            needs_x = not (isinstance(field.data_type, ir.Struct)
                                           and not field.data_type.all_fields)
                            ref_x = 'ref x' if needs_x else '_'
                            with self.block(u'{}::{}({}) =>'.format(
                                    type_name, variant_name, ref_x)):
                                if isinstance(ultimate_type, ir.Union) or \
                                        (isinstance(ultimate_type, ir.Struct)
                                         and ultimate_type.has_enumerated_subtypes()):
                                    # Inner type is a union or polymorphic struct; need to always
                                    # emit another nesting level.
                                    self.emit(u'// union or polymporphic struct')
                                    self.emit(u'let mut s = serializer.serialize_struct("{}", 2)?;'
                                              .format(union.name))
                                    self.emit(u's.serialize_field(".tag", "{}")?;'
                                              .format(field.name))
                                    self.emit(u's.serialize_field("{}", x)?;'.format(field.name))
                                    self.emit(u's.end()')
                                elif isinstance(ir.unwrap_aliases(field.data_type)[0], ir.Nullable):
                                    self.emit(u'// nullable (struct or primitive)')
                                    # If it's nullable and the value is None, just emit the tag and
                                    # nothing else, otherwise emit the fields directly at the same
                                    # level.
                                    num_fields = 1 if ir.is_primitive_type(ultimate_type) \
                                        else len(ultimate_type.all_fields) + 1
                                    self.emit(u'let n = if x.is_some() {{ {} }} else {{ 1 }};'
                                              .format(num_fields + 1))
                                    self.emit(u'let mut s = serializer.serialize_struct("{}", n)?;'
                                              .format(union.name))
                                    self.emit(u's.serialize_field(".tag", "{}")?;'
                                              .format(field.name))
                                    with self.block(u'if let &Some(ref x) = x'):
                                        if ir.is_primitive_type(ultimate_type):
                                            self.emit(u's.serialize_field("{}", &x)?;'
                                                      .format(field.name))
                                        else:
                                            self.emit(u'x.internal_serialize::<S>(&mut s)?;')
                                    self.emit(u's.end()')
                                elif isinstance(ultimate_type, ir.Struct):
                                    self.emit(u'// struct')
                                    self.emit(u'let mut s = serializer.serialize_struct("{}", {})?;'
                                              .format(union.name,
                                                      len(ultimate_type.all_fields) + 1))
                                    self.emit(u's.serialize_field(".tag", "{}")?;'
                                              .format(field.name))
                                    if ultimate_type.all_fields:
                                        self.emit(u'x.internal_serialize::<S>(&mut s)?;')
                                    self.emit(u's.end()')
                                else:
                                    self.emit(u'// primitive')
                                    self.emit(u'let mut s = serializer.serialize_struct("{}", 2)?;'
                                              .format(union.name))
                                    self.emit(u's.serialize_field(".tag", "{}")?;'
                                              .format(field.name))
                                    self.emit(u's.serialize_field("{}", x)?;'.format(field.name))
                                    self.emit(u's.end()')
                    if not union.closed:
                        self.emit(u'{}::Other => Err(::serde::ser::Error::custom('
                                  u'"cannot serialize \'Other\' variant"))'
                                  .format(type_name))
        self.emit()

    # Helpers

    def _emit_doc(self, doc_string):
        if doc_string is not None:
            self.emit_wrapped_text(doc_string, prefix=u'/// ', width=100)

    def _impl_deserialize(self, type_name):
        return nested(self.block(u'impl<\'de> ::serde::de::Deserialize<\'de> for {}'.format(
                type_name)),
            self.emit_rust_function_def(
                u'deserialize<D: ::serde::de::Deserializer<\'de>>',
                [u'deserializer: D'],
                u'Result<Self, D::Error>'))

    def _impl_serialize(self, type_name):
        return nested(
            self.block(u'impl ::serde::ser::Serialize for {}'.format(type_name)),
            self.emit_rust_function_def(
                u'serialize<S: ::serde::ser::Serializer>',
                [u'&self', u'serializer: S'],
                u'Result<S::Ok, S::Error>'))

    def _impl_default_for_struct(self, struct):
        struct_name = self.struct_name(struct)
        with self.block(u'impl Default for {}'.format(struct_name)):
            with self.emit_rust_function_def(u'default', [], u'Self'):
                with self.block(struct_name):
                    for field in struct.all_fields:
                        self.emit(u'{}: {},'.format(
                            self.field_name(field), self._default_value(field)))

    def _impl_struct(self, struct):
        return self.block(u'impl {}'.format(self.struct_name(struct)))

    def _emit_new_for_struct(self, struct):
        struct_name = self.struct_name(struct)
        with self.emit_rust_function_def(
                u'new',
                [u'{}: {}'.format(self.field_name(field), self._rust_type(field.data_type))
                    for field in struct.all_required_fields],
                u'Self',
                access=u'pub'):
            with self.block(struct_name):
                for field in struct.all_required_fields:
                    # shorthand assignment
                    self.emit(u'{},'.format(self.field_name(field)))
                for field in struct.all_optional_fields:
                    self.emit(u'{}: {},'.format(
                        self.field_name(field),
                        self._default_value(field)))

        for field in struct.all_optional_fields:
            self.emit()
            field_name = self.field_name(field)
            with self.emit_rust_function_def(
                    u'with_{}'.format(field_name),
                    [u'mut self', u'value: {}'.format(self._rust_type(field.data_type))],
                    u'Self',
                    access=u'pub'):
                self.emit(u'self.{} = value;'.format(field_name))
                self.emit(u'self')

    def _default_value(self, field):
        if isinstance(field.data_type, ir.Nullable):
            return u'None'
        elif ir.is_numeric_type(ir.unwrap_aliases(field.data_type)[0]):
            return field.default
        elif isinstance(field.default, ir.TagRef):
            default_variant = None
            for variant in field.default.union_data_type.all_fields:
                if variant.name == field.default.tag_name:
                    default_variant = variant
            if default_variant is None:
                raise RuntimeError('ERROR: didn\'t find matching variant of {}: {}'
                                   .format(field.data_type.name, field.default.tag_name))
            return u'{}::{}'.format(
                self._rust_type(field.default.union_data_type),
                self.enum_variant_name(default_variant))
        elif isinstance(field.data_type, ir.Boolean):
            if field.default:
                return u'true'
            else:
                return u'false'
        elif isinstance(field.data_type, ir.String):
            if not field.default:
                return u'String::new()'
            else:
                return u'"{}".to_owned()'.format(field.default)
        else:
            print(u'WARNING: unhandled default value {}'.format(field.default))
            print(u'    in field: {}'.format(field))
            if isinstance(field.data_type, ir.Alias):
                print(u'    unwrapped alias: {}'.format(ir.unwrap_aliases(field.data_type)[0]))
            return field.default

    def _needs_explicit_default(self, field):
        return field.has_default \
                and not (isinstance(field, ir.Nullable)
                         or (isinstance(field.data_type, ir.Boolean) and not field.default))

    def _impl_error(self, type_name):
        with self.block(u'impl ::std::error::Error for {}'.format(type_name)):
            with self.emit_rust_function_def(u'description', [u'&self'], u'&str'):
                self.emit(u'"{}"'.format(type_name))
        self.emit()
        with self.block(u'impl ::std::fmt::Display for {}'.format(type_name)):
            with self.emit_rust_function_def(
                    u'fmt',
                    [u'&self', u'f: &mut ::std::fmt::Formatter'],
                    u'::std::fmt::Result'):
                self.emit(u'write!(f, "{:?}", *self)')
        self.emit()

    # Naming Rules

    def _rust_type(self, typ):
        if isinstance(typ, ir.Nullable):
            return u'Option<{}>'.format(self._rust_type(typ.data_type))
        elif isinstance(typ, ir.Void):
            return u'()'
        elif isinstance(typ, ir.Bytes):
            return u'Vec<u8>'
        elif isinstance(typ, ir.Int32):
            return u'i32'
        elif isinstance(typ, ir.UInt32):
            return u'u32'
        elif isinstance(typ, ir.Int64):
            return u'i64'
        elif isinstance(typ, ir.UInt64):
            return u'u64'
        elif isinstance(typ, ir.Float32):
            return u'f32'
        elif isinstance(typ, ir.Float64):
            return u'f64'
        elif isinstance(typ, ir.Boolean):
            return u'bool'
        elif isinstance(typ, ir.String):
            return u'String'
        elif isinstance(typ, ir.Timestamp):
            return u'String /*Timestamp*/'  # TODO
        elif isinstance(typ, ir.List):
            return u'Vec<{}>'.format(self._rust_type(typ.data_type))
        elif isinstance(typ, ir.Map):
            return u'::std::collections::HashMap<{}, {}>'.format(
                self._rust_type(typ.key_data_type),
                self._rust_type(typ.value_data_type))
        elif isinstance(typ, ir.Alias):
            if typ.namespace.name == self._current_namespace:
                return self.alias_name(typ)
            else:
                return u'super::{}::{}'.format(
                    self.namespace_name(typ.namespace),
                    self.alias_name(typ))
        elif isinstance(typ, ir.UserDefined):
            if isinstance(typ, ir.Struct):
                name = self.struct_name(typ)
            elif isinstance(typ, ir.Union):
                name = self.enum_name(typ)
            else:
                raise RuntimeError(u'ERROR: user-defined type "{}" is neither Struct nor Union???'
                                   .format(typ))
            if typ.namespace.name == self._current_namespace:
                return name
            else:
                return u'super::{}::{}'.format(
                    self.namespace_name(typ.namespace),
                    name)
        else:
            raise RuntimeError(u'ERROR: unhandled type "{}"'.format(typ))
