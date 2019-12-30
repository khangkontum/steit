use crate::gen::*;

pub struct CSharpGenerator {
    namespace: String,
    out_dir: String,
}

impl CSharpGenerator {
    pub fn new(namespace: impl Into<String>, out_dir: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            out_dir: out_dir.into(),
        }
    }
}

impl Generator for CSharpGenerator {
    fn out_dir(&self) -> &str {
        &self.out_dir
    }

    fn indent_size(&self) -> usize {
        4
    }

    fn generate_struct(&self, r#struct: Struct, _is_variant: bool, writer: &mut Writer) {
        let name = r#struct.name;
        let var_name = string_utils::uncap_first_char(name);

        let fields: Vec<_> = r#struct
            .fields
            .iter()
            .map(CSharpField::with_field)
            .collect();

        writer
            .writeln("using System;")
            .writeln("using System.Collections.Generic;")
            .newline()
            .writeln("using Steit;")
            .writeln("using Steit.Reader;")
            .newline()
            .writeln(format!("namespace {} {{", self.namespace))
            .indent()
            .writeln(format!("public sealed class {} : State {{", name))
            .indent();

        // Declare listener lists
        for field in &fields {
            writer.writeln(format!(
                "private static IList<Listener<{0}>> {1}Listeners = new List<Listener<{0}>>();",
                field.ty, field.lower_camel_case_name,
            ));
        }

        writer
            .newline()
            .writeln("public Path Path { get; private set; }")
            .newline();

        // Declare properties
        for field in &fields {
            writer.writeln(format!(
                "public {} {} {{ get; private set; }}",
                field.ty, field.upper_camel_case_name,
            ));
        }

        writer
            .newline()
            .writeln(format!("public {}(Path path = null) {{", name))
            .indent()
            .writeln("this.Path = path != null ? path : Path.Root;");

        // Initiate nested states
        for field in &fields {
            if let FieldType::Meta(_) = field.raw.ty {
                writer.writeln(format!(
                    "this.{} = new {}(this.Path.Nested({}));",
                    field.upper_camel_case_name, field.ty, field.raw.tag,
                ));
            }
        }

        writer
            .outdent()
            .writeln("}")
            .newline()
            .writeln(format!(
                "public delegate void Listener<T>(T newValue, T oldValue, {} container);",
                name,
            ))
            .newline();

        // Support adding listeners
        for field in &fields {
            writer.writeln(format!(
                "public static int OnUpdate{}(Listener<{}> listener) {{ return Utils.Add({}Listeners, listener); }}",
                field.upper_camel_case_name, field.ty, field.lower_camel_case_name,
            ));
        }

        writer.newline();

        // Support removing listeners
        for field in &fields {
            writer.writeln(format!(
                "public static void Remove{}Listener(Listener<{}> listener) {{ {}Listeners.Remove(listener); }}",
                field.upper_camel_case_name, field.ty, field.lower_camel_case_name,
            ));
        }

        writer.newline();

        // Support removing listeners at specific indices
        for field in &fields {
            writer.writeln(format!(
                "public static void Remove{}ListenerAt(int index) {{ {}Listeners.RemoveAt(index); }}",
                field.upper_camel_case_name, field.lower_camel_case_name,
            ));
        }

        writer.newline();

        // Support clearing listener lists
        for field in &fields {
            writer.writeln(format!(
                "public static void Clear{}Listeners() {{ {}Listeners.Clear(); }}",
                field.upper_camel_case_name, field.lower_camel_case_name,
            ));
        }

        writer
            .newline()
            .writeln("public static void ClearAllListeners() {")
            .indent();

        // Support clearing all listener lists
        for field in &fields {
            writer.writeln(format!("{}Listeners.Clear();", field.lower_camel_case_name));
        }

        writer
            .outdent()
            .writeln("}")
            .newline()
            .writeln(format!(
                "public static {} Deserialize(StateReader reader, Path path = null) {{",
                name,
            ))
            .indent()
            .writeln(format!("var {} = new {}(path);", var_name, name))
            .writeln(format!(
                "{}.ReplaceAll(reader.Nested((int) reader.ReadUInt32()), shouldNotify: false);",
                var_name,
            ))
            .writeln(format!("return {};", var_name))
            .outdent()
            .writeln("}")
            .newline()
            .writeln("public override State Nested(UInt16 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return nested states
        for field in &fields {
            if let FieldType::Meta(_) = field.raw.ty {
                writer.writeln(format!(
                    "case {}: return this.{};",
                    field.raw.tag, field.upper_camel_case_name,
                ));
            }
        }

        writer
            .writeln("default: return null;")
            .outdent()
            .writeln("}")
            .outdent()
            .writeln("}")
            .newline()
            .writeln("protected override Int16 WireType(UInt16 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for field in &fields {
            if let FieldType::Meta(_) = field.raw.ty {
                writer.writeln(format!(
                    "case {}: return StateReader.WIRE_TYPE_SIZED;",
                    field.raw.tag,
                ));
            } else {
                writer.writeln(format!(
                    "case {}: return StateReader.WIRE_TYPE_VARINT;",
                    field.raw.tag,
                ));
            }
        }

        writer
            .writeln("default: return -1;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("protected override void ReplaceAt(UInt16 tag, Byte wireType, StateReader reader, bool shouldNotify) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Replace fields and notify listeners
        for field in &fields {
            if let FieldType::Meta(_) = field.raw.ty {
                writer.writeln(format!(
                    "case {0}: this.{1} = this.Notify({1}.Deserialize(reader, this.Path.Nested({0})), this.{1}, shouldNotify, {2}Listeners); break;",
                    field.raw.tag,
                    field.upper_camel_case_name,
                    field.lower_camel_case_name,
                ));
            } else {
                writer.writeln(format!(
                    "case {0}: this.{1} = this.Notify(reader.Read{3}(), this.{1}, shouldNotify, {2}Listeners); break;",
                    field.raw.tag,
                    field.upper_camel_case_name,
                    field.lower_camel_case_name,
                    field.ty,
                ));
            }
        }

        writer
            .writeln("default: reader.SkipWireTyped(wireType); break;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("private T Notify<T>(T newValue, T oldValue, bool shouldNotify, IList<Listener<T>> listeners) {")
            .indent_writeln("if (shouldNotify) {")
            .indent_writeln("foreach (var listener in listeners) {")
            .indent_writeln("listener(newValue, oldValue, this);")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("return newValue;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .outdent_writeln("}");
    }

    fn generate_enum(&self, _enum: Enum, _writer: &mut Writer) {}
}

struct CSharpField {
    raw: &'static Field,
    upper_camel_case_name: String,
    lower_camel_case_name: String,
    ty: String,
}

impl CSharpField {
    pub fn with_field(field: &'static Field) -> Self {
        Self {
            raw: field,
            upper_camel_case_name: string_utils::to_camel_case(field.name, true),
            lower_camel_case_name: string_utils::to_camel_case(field.name, false),
            ty: get_type(field.ty),
        }
    }
}

fn get_type(ty: &'static FieldType) -> String {
    match ty {
        FieldType::Primitive(name) => match *name {
            "u8" => "UInt8".to_owned(),
            "u16" => "UInt16".to_owned(),
            "u32" => "UInt32".to_owned(),
            "u64" => "UInt64".to_owned(),
            "i8" => "Int8".to_owned(),
            "i16" => "Int16".to_owned(),
            "i32" => "Int32".to_owned(),
            "i64" => "Int64".to_owned(),
            "bool" => "Boolean".to_owned(),
            _ => name.to_string(),
        },

        FieldType::Meta(meta) => match meta {
            Meta::Struct(Struct { name, .. }) => name.to_string(),
            Meta::Enum(Enum { name, .. }) => name.to_string(),
            Meta::List(field) => format!("StateList<{}>", get_type(field.ty)),
            Meta::Map(field) => format!("StateDictionary<{}>", get_type(field.ty)),
        },
    }
}
