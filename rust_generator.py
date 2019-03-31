from loader import *
from model import *
import re

model = load_model("output.xml")

# https://stackoverflow.com/a/41510011
RE_WORDS = re.compile(r'''
    # Find words in a string. Order matters!
    [A-Z]+(?=[A-Z][a-z]) |  # All upper case before a capitalized word
    [A-Z]?[a-z]+ |  # Capitalized words / all lower case
    [A-Z]+ |  # All upper case
    \d+  # Numbers
''', re.VERBOSE)

def pascal_case_split(identifier):
    matches = re.finditer(RE_WORDS, identifier)
    return [m.group(0) for m in matches]

def remove_prefix(s: str, p: str) -> str:
    if s.startswith(p):
        return s[len(p):]
    else:
        return s

def upper_snake_to_pascal(s: str) -> str:
    return "".join([x[0].upper() + x[1:] for x in s.lower().split("_")])

def camel_to_snake(s: str) -> str:
    tokens = pascal_case_split(s)
    return "_".join(tokens).lower()

def format_enum_value(s: str, p: str) -> str:
    tokens = pascal_case_split(p)
    prefix1 = "_".join(tokens).upper() + "_"
    prefix2 = "_".join(tokens[:-1]).upper() + "_"
    attempt = remove_prefix(s, prefix1)
    if attempt == s:
        attempt = remove_prefix(s, prefix2)
    if attempt[0].isdigit():
        attempt = "K_" + attempt
    return attempt.upper()

def format_bitmask_value(s: str, p: str) -> str:
    p = p.replace("FlagBits", "")
    tokens = pascal_case_split(p)
    prefix1 = "_".join(tokens).upper() + "_"
    prefix2 = "_".join(tokens[:-1]).upper() + "_"
    attempt = remove_prefix(s, prefix1)
    if attempt == s:
        attempt = remove_prefix(s, prefix2)
    if attempt[0].isdigit():
        attempt = "K_" + attempt
    return attempt.upper()

def make_default_value_type(name: str, t: Type) -> str:
    if isinstance(t, PointerType):
        if t.inner.is_const:
            return "core::ptr::null()"
        else:
            return "core::ptr::null_mut()"
    elif isinstance(t, ArrayType):
        default_value = make_default_value_type("", t.inner)
        return "[" + (", ".join([default_value] * int(t.length))) + "]"
    elif isinstance(t, TypeReference):
        if t.name == "float":
            return "0.0"
        elif name.startswith("pfn"):
            return "unsafe { core::mem::zeroed() }"
        elif t.name.startswith("Vk"):
            return to_rust_type(t.name) + "::default()"
        else:
            return "0"
    return "0"

def make_default_value(m: StructureMember) -> str:
    if m.id.name == "sType" and m.default_value:
        return "VkStructureType::" + remove_prefix(m.default_value, "VK_STRUCTURE_TYPE_")
    else:
        return make_default_value_type(m.id.name, m.id.type)

def to_rust_type(name: str) -> str:
    if name == "uint8_t":
        return "u8"
    elif name == "int8_t":
        return "i8"
    elif name == "uint16_t":
        return "u16"
    elif name == "int16_t":
        return "i16"
    elif name == "uint32_t":
        return "u32"
    elif name == "int32_t":
        return "i32"
    elif name == "uint64_t":
        return "u64"
    elif name == "int64_t":
        return "i64"
    elif name == "uintptr_t":
        return "usize"
    elif name == "float":
        return "f32"
    elif name == "char":
        return "u8"
    elif name == "void":
        return "core::ffi::c_void"
    elif name == "size_t":
        return "usize"
    elif name.startswith("PFN_vk"):
        return "PfnVk" + remove_prefix(name, "PFN_vk")
    else:
        return name

def to_rust_type_deep(t: Type) -> str:
    def to_rust_type_deep_inner(t: Type) -> str:
        if isinstance(t, TypeReference):
            if t.is_const:
                return "const " + to_rust_type(t.name)
            else:
                return "mut " + to_rust_type(t.name)
        elif isinstance(t, PointerType):
            inner = to_rust_type_deep_inner(t.inner)
            if t.is_const:
                return "const *" + inner
            else:
                return "mut *" + inner
        elif isinstance(t, ArrayType):
            inner = to_rust_type_deep(t.inner)
            return "[%s; %d]" % (inner, t.length)
        else:
            return ""
    t_str = to_rust_type_deep_inner(t)
    return remove_prefix(remove_prefix(t_str, "mut "), "const ")

fp = open("src/types.rs", "w+")

fp.write("pub type HINSTANCE = usize;\n")
fp.write("pub type HWND = usize;\n")
fp.write("\n")

for t in model["integer_constants"]:
    fp.write("pub const %s: u%d = %d;\n" % (t.name, t.size, t.value))
fp.write("\n")

for t in model["real_constants"]:
    fp.write("pub const %s: f%d = %f;\n" % (t.name, t.size, t.value))
fp.write("\n")

for t in model["string_constants"]:
    fp.write("pub const %s: &str = \"%s\";\n" % (t.name, t.value))
    fp.write("pub const %s__C: &[u8] = b\"%s\\0\";\n" % (t.name, t.value))
fp.write("\n")

for t in model["base_types"]:
    fp.write("pub type %s = %s;\n" % (t.name, to_rust_type(t.alias)))
fp.write("\n")

for t in model["bitmask_types"]:
    fp.write("pub type %s = %s;\n" % (t.name, to_rust_type(t.alias)))
fp.write("\n")

for t in model["handle_types"]:
    fp.write("#[repr(transparent)]\n")
    fp.write("#[derive(Default, Copy, Clone, PartialEq, Eq)]\n")
    type_name = t.name
    rust_type = to_rust_type(t.type)
    fp.write("pub struct %s(%s);\n" % (type_name, rust_type))
    fp.write("impl %s {\n" % type_name)
    fp.write("    #[inline]\n")
    fp.write("    pub fn null() -> Self {\n")
    fp.write("        Self(0)\n")
    fp.write("    }\n\n")
    fp.write("    #[inline]\n")
    fp.write("    pub fn from_raw(r: %s) -> Self {\n" % rust_type)
    fp.write("        Self(r)\n")
    fp.write("    }\n\n")
    fp.write("    #[inline]\n")
    fp.write("    pub fn as_raw(&self) -> %s {\n" % rust_type)
    fp.write("        self.0\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["alias_types"]:
    fp.write("pub type %s = %s;\n" % (t.name, to_rust_type(t.alias)))
fp.write("\n")

for t in model["enum_types"]:
    if t.type != "enum":
        continue

    enum_name = t.name
    fp.write("#[repr(transparent)]\n")
    fp.write("#[derive(Default, PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]\n")
    fp.write("pub struct %s(u32);\n" % enum_name)
    fp.write("impl %s {\n" % enum_name)
    for v in t.values:
        value = t.values[v]
        v = format_enum_value(v, enum_name)
        if value < 0:
            fp.write("    pub const %s: %s = %s(%di32 as u32);\n" % (v, enum_name, enum_name, value))
        else:
            fp.write("    pub const %s: %s = %s(%d);\n" % (v, enum_name, enum_name, value))
    fp.write("}\n\n")

for t in model["enum_types"]:
    if t.type != "bitmask":
        continue

    enum_name = t.name
    fp.write("#[repr(transparent)]\n")
    fp.write("#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]\n")
    fp.write("pub struct %s(VkFlags);\n" % enum_name)
    fp.write("impl %s {\n" % enum_name)
    for v in t.values:
        value = t.values[v]
        v = format_bitmask_value(v, enum_name)
        if value < 0:
            fp.write("    pub const %s: %s = %s(%di32 as u32);\n" % (v, enum_name, enum_name, value))
        else:
            fp.write("    pub const %s: %s = %s(%d);\n" % (v, enum_name, enum_name, value))
    fp.write("\n")
    fp.write("    #[inline]\n")
    fp.write("    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitOr for %s {\n" % enum_name)
    fp.write("    type Output = %s;\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitOrAssign for %s {\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitAnd for %s {\n" % enum_name)
    fp.write("    type Output = %s;\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitAndAssign for %s {\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitXor for %s {\n" % enum_name)
    fp.write("    type Output = %s;\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitXorAssign for %s {\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }\n")
    fp.write("}\n\n")

for t in model["structure_types"]:
    struct_name = t.name
    fp.write("#[repr(C)]\n")
    fp.write("#[derive(Copy, Clone)]\n")
    fp.write("pub struct %s {\n" % struct_name)
    for m in t.members:
        member_name = m.id.name
        member_name = camel_to_snake(member_name)
        if member_name == "type":
            member_name = "kind"
        fp.write("    pub %s: %s,\n" % (member_name, to_rust_type_deep(m.id.type)))
    fp.write("}\n\n")
    fp.write("impl Default for %s {\n" % struct_name)
    fp.write("    fn default() -> Self {\n")
    fp.write("        Self {\n")
    for m in t.members:
        member_name = camel_to_snake(m.id.name)
        if member_name == "type":
            member_name = "kind"
        default_value = make_default_value(m)
        fp.write("            %s: %s,\n" % (member_name, default_value))
    fp.write("        }\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["union_types"]:
    struct_name = t.name
    fp.write("#[repr(C)]\n")
    fp.write("#[derive(Copy, Clone)]\n")
    fp.write("pub union %s {\n" % struct_name)
    for m in t.members:
        member_name = m.name
        member_name = camel_to_snake(member_name)
        if member_name == "type":
            member_name = "kind"
        fp.write("    pub %s: %s,\n" % (member_name, to_rust_type_deep(m.type)))
    fp.write("}\n\n")
    fp.write("impl Default for %s {\n" % struct_name)
    fp.write("    fn default() -> Self {\n")
    fp.write("        unsafe { core::mem::zeroed() }\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["function_pointer_types"]:
    fn_name = "PfnVk" + t.name[6:]
    fp.write("pub type %s = extern \"system\" fn(\n" % fn_name)
    for a in t.prototype.arguments:
        fp.write("    %s: %s,\n" % (camel_to_snake(a.id.name), to_rust_type_deep(a.id.type)))
    fp.write(")")
    if not isinstance(t.prototype.return_type, TypeReference) or t.prototype.return_type.name != "void":
        fp.write(" -> %s" % to_rust_type_deep(t.prototype.return_type))
    fp.write(";\n\n")

for t in model["commands"]:
    fn_name = t.name
    fp.write("pub type PfnVk%s = extern \"system\" fn(\n" % fn_name[2:])
    for a in t.prototype.arguments:
        arg_name = camel_to_snake(a.id.name)
        if arg_name == "type":
            arg_name = "kind"
        fp.write("    %s: %s,\n" % (arg_name, to_rust_type_deep(a.id.type)))
    fp.write(")")
    if not isinstance(t.prototype.return_type, TypeReference) or t.prototype.return_type.name != "void":
        fp.write(" -> %s" % to_rust_type_deep(t.prototype.return_type))
    fp.write(";\n\n")

fp.close()

fp = open("src/commands.rs", "w+")
fp.write("use crate::types::*;\n\n")

def write_commands_collection(fp, type: str, struct_name: str):
    fp.write("pub struct %s {\n" % struct_name)
    for t in model["commands"]:
        if t.type != type:
            continue
        fp.write("    pfn_%s: PfnVk%s,\n" % (camel_to_snake(t.name[2:]), t.name[2:]))
    fp.write("}\n\n")
    fp.write("impl %s {\n" % struct_name)
    fp.write("    pub fn load(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {\n")
    fp.write("        %s {\n" % struct_name)
    for t in model["commands"]:
        if t.type != type:
            continue
        fp.write("            pfn_%s: unsafe { core::mem::transmute(load_fn(b\"%s\\0\")) },\n" % (camel_to_snake(t.name[2:]), t.name))
    fp.write("        }\n")
    fp.write("    }\n")
    for t in model["commands"]:
        if t.type != type:
            continue
        fp.write("\n    pub unsafe fn %s(&self" % camel_to_snake(t.name[2:]))
        for a in t.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"
            fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep(a.id.type)))
        fp.write(")")
        if not isinstance(t.prototype.return_type, TypeReference) or t.prototype.return_type.name != "void":
            fp.write(" -> %s" % to_rust_type_deep(t.prototype.return_type))
        fp.write(" {\n")
        inner_fn_name = "pfn_" + camel_to_snake(t.name[2:])
        fp.write("        (self.%s)(" % inner_fn_name)
        for a in t.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"
            fp.write("\n            %s," % arg_name)
        fp.write(")\n")
        fp.write("    }\n")
    fp.write("}\n\n")

write_commands_collection(fp, "static", "StaticCommands")
write_commands_collection(fp, "entry", "EntryCommands")
write_commands_collection(fp, "instance", "InstanceCommands")
write_commands_collection(fp, "device", "DeviceCommands")

fp.close()

fp = open("src/builders.rs", "w+")
fp.write("use crate::types::*;\n\n")

for t in model["structure_types"]:
    struct_name = t.name
    fp.write("pub struct %sBuilder {\n" % struct_name)
    fp.write("    s: %s,\n" % struct_name)
    fp.write("}\n\n")
    fp.write("impl %sBuilder {\n" % struct_name)
    fp.write("    pub fn new() -> Self {\n")
    fp.write("        Self {\n")
    fp.write("            s: %s::default(),\n" % struct_name)
    fp.write("        }\n")
    fp.write("    }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::Deref for %sBuilder {\n" % struct_name)
    fp.write("    type Target = %s;\n\n" % struct_name)
    fp.write("    fn deref(&self) -> &Self::Target {\n")
    fp.write("        &self.s\n")
    fp.write("    }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::DerefMut for %sBuilder {\n" % struct_name)
    fp.write("    fn deref_mut(&mut self) -> &mut Self::Target {\n")
    fp.write("        &mut self.s\n")
    fp.write("    }\n")
    fp.write("}\n\n")

fp.close()