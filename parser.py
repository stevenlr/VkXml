from typing import *
import re
import xml.etree.ElementTree as xml
from xml.dom import minidom
from model import *

VERSION = "1.0"
WANTED_EXTENSIONS = ["VK_KHR_win32_surface", "VK_KHR_swapchain", "VK_EXT_debug_utils"]
OUTPUT = "output.xml"

def stringify_tag_except_comment(tag):
    if tag.tag == "comment": return ""

    elements = []

    if isinstance(tag.text, str):
        elements.append(tag.text)

    for e in tag:
        if isinstance(e, str):
            elements.append(e.strip())
        else:
            elements.append(stringify_tag_except_comment(e))

        if isinstance(e.tail, str):
            elements.append(e.tail.strip())

    return " ".join((" ".join(elements)).split()) + " "

class TokenString:
    def __init__(self, s: str):
        self.s = s.strip()
        self.id_regex = re.compile("^[a-zA-Z_][a-zA-Z0-9_]*")

    def maybe_eat_token_end(self, token: str) -> bool:
        if self.s.endswith(token):
            self.s = self.s[:-len(token)].strip()
            return True
        else:
            return False

    def maybe_eat_tokens_end(self, tokens: List[str]) -> bool:
        for token in reversed(tokens):
            if not self.maybe_eat_token_end(token):
                return False
        return True

    def maybe_eat_token(self, token: str) -> bool:
        if self.s.startswith(token):
            self.s = self.s[len(token):].strip()
            return True
        else:
            return False

    def maybe_eat_tokens(self, tokens: List[str]) -> bool:
        for token in tokens:
            if not self.maybe_eat_token(token):
                return False
        return True

    def eat_next_identifier(self) -> str:
        match = self.id_regex.search(self.s)
        if match == None:
            raise Exception("Not an identifier")
        else:
            identifier = match.group(0)
            self.s = self.s[len(identifier):].strip()
            return identifier

    def eat_next_until(self, limit: str) -> str:
        index = self.s.find(limit)
        if index == -1:
            raise Exception("String limit not found")
        token = self.s[:index].strip()
        self.s = self.s[index + len(limit):].strip()
        return token

    def is_finished(self) -> bool:
        return len(self.s) == 0

def is_int(s: str) -> bool:
    try:
        int(s, 0)
        return True
    except Exception:
        return False

def version_to_int(s: str) -> int:
    version = 0
    for i in [int(x) for x in s.split(".")]:
        version = version * 1000 + i
    return version

class Feature:
    def __init__(self, name: str, version: str):
        self.name = name
        self.version = version
        self.version_int = version_to_int(version)

class Extension:
    def __init__(self, name: str, number: int, dependencies: List[str], type: str):
        self.name = name
        self.number = number
        self.dependencies = dependencies
        self.type = type

ROOT = xml.parse("vk.xml").getroot()
PLATFORM_TYPES:         Dict[str, Entity] = {}
BASE_TYPES:             Dict[str, BaseType] = {}
HANDLE_TYPES:           Dict[str, HandleType] = {}
ALIAS_TYPES:            Dict[str, AliasType] = {}
ENUM_TYPES:             Dict[str, EnumType] = {}
BITMASK_TYPES:          Dict[str, BitmaskType] = {}
STRUCTURE_TYPES:        Dict[str, StructureType] = {}
UNION_TYPES:            Dict[str, UnionType] = {}
FUNCTION_POINTER_TYPES: Dict[str, FunctionPointerType] = {}
CONSTANTS:              Dict[str, Constant] = {}
COMMANDS:               Dict[str, Command] = {}
EMPTY:                  Dict[str, Entity] = {}
FEATURES:               Dict[str, Feature] = {}
EXTENSIONS:             Dict[str, Extension] = {}

def add_platform_type(name: str):
    PLATFORM_TYPES[name] = Entity(name)

add_platform_type("void")
add_platform_type("char")
add_platform_type("uint8_t")
add_platform_type("int8_t")
add_platform_type("uint16_t")
add_platform_type("int16_t")
add_platform_type("uint32_t")
add_platform_type("int32_t")
add_platform_type("uint64_t")
add_platform_type("int64_t")
add_platform_type("float")
add_platform_type("size_t")
add_platform_type("uintptr_t")

def find_entity(name: str) -> Optional[Entity]:
    if name in BASE_TYPES:
        return BASE_TYPES[name]
    elif name in HANDLE_TYPES:
        return HANDLE_TYPES[name]
    elif name in ALIAS_TYPES:
        return ALIAS_TYPES[name]
    elif name in ENUM_TYPES:
        return ENUM_TYPES[name]
    elif name in BITMASK_TYPES:
        return BITMASK_TYPES[name]
    elif name in STRUCTURE_TYPES:
        return STRUCTURE_TYPES[name]
    elif name in UNION_TYPES:
        return UNION_TYPES[name]
    elif name in FUNCTION_POINTER_TYPES:
        return FUNCTION_POINTER_TYPES[name]
    elif name in CONSTANTS:
        return CONSTANTS[name]
    elif name in COMMANDS:
        return COMMANDS[name]
    elif name in EMPTY:
        return EMPTY[name]
    elif name in PLATFORM_TYPES:
        return PLATFORM_TYPES[name]
    else:
        return None

def parse_basetype(type):
    name = type.find("./name").text
    base = type.find("./type").text
    BASE_TYPES[name] = BaseType(name, base)

def parse_handle(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = AliasType(type.attrib["name"], type.attrib["alias"])
    else:
        handle_type = type.find("./type").text
        name = type.find("./name").text
        if handle_type == "VK_DEFINE_HANDLE":
            HANDLE_TYPES[name] = HandleType(name, True)
        elif handle_type == "VK_DEFINE_NON_DISPATCHABLE_HANDLE":
            HANDLE_TYPES[name] = HandleType(name, False)
        else:
            print("Unknown handle type %s for %s" % (handle_type, name))

def parse_enum(type):
    name = type.attrib["name"]
    if "alias" in type.attrib:
        ALIAS_TYPES[name] = AliasType(name, type.attrib["alias"])
    else:
        enums = ROOT.find("./enums[@name='%s']" % name)
        if enums != None:
            enum = EnumType(name, enums.attrib["type"])
            enum_type = enums.attrib["type"]
            values = {}
            for e in enums.findall("./enum"):
                if "alias" in e.attrib:
                    enum.add_value(e.attrib["name"], enum.get_value(e.attrib["alias"]))
                elif "bitpos" in e.attrib:
                    enum.add_value(e.attrib["name"], 1 << int(e.attrib["bitpos"]))
                else:
                    enum.add_value(e.attrib["name"], int(e.attrib["value"], 0))
            ENUM_TYPES[name] = enum

def parse_enum_extends(tag, extnumber: int) -> Entity:
    base_enum_name = tag.attrib["extends"]

    if not base_enum_name in ENUM_TYPES:
        raise Exception("Unknown enum to extend %s" % base_enum_name)

    name = tag.attrib["name"]
    value = 0

    if "extnumber" in tag.attrib:
        extnumber = int(tag.attrib["extnumber"], 0)

    if "bitpos" in tag.attrib:
        value = 1 << int(tag.attrib["bitpos"])
    else:
        value = 1000000000 + 1000 * (extnumber - 1) + int(tag.attrib["offset"], 0)

    if "dir" in tag.attrib and tag.attrib["dir"] == "-":
        value = -value

    ENUM_TYPES[base_enum_name].add_value(name, value)
    return ENUM_TYPES[base_enum_name]

def parse_bitmask(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = AliasType(type.attrib["name"], type.attrib["alias"])
    else:
        name = type.find("./name").text
        if "requires" in type.attrib:
            BITMASK_TYPES[name] = BitmaskType(name, type.attrib["requires"])
        else:
            ALIAS_TYPES[name] = AliasType(name, "VkFlags")

def parse_type_reference(tokens: TokenString) -> Type:
    tokens.maybe_eat_token("struct") # Ignore this!
    is_const = tokens.maybe_eat_token("const")
    tokens.maybe_eat_token("struct") # Ignore this!
    type_name = tokens.eat_next_identifier()
    is_const = is_const or tokens.maybe_eat_token("const")
    type: Any = TypeReference(type_name, is_const)

    while tokens.maybe_eat_token("*"):
        is_const = tokens.maybe_eat_token("const")
        type = PointerType(type, is_const)

    return type

def parse_typed_entity(s: str) -> TypedIdentifier:
    tokens = TokenString(s)
    type = parse_type_reference(tokens)
    entity_name = tokens.eat_next_identifier()

    if tokens.maybe_eat_token("["):
        length: Any = tokens.eat_next_until("]")
        if not is_int(length):
            e = ROOT.find("enums/enum[@name='%s']" % length)
            if e == None:
                raise Exception("Cannot find enum")
            else:
                length = int(e.attrib["value"], 0)
        type = ArrayType(type, length)

    if not tokens.is_finished():
        raise Exception("Didn't finish parsing properly")

    return TypedIdentifier(entity_name, type)

def parse_struct(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = AliasType(type.attrib["name"], type.attrib["alias"])
        return

    struct_name = type.attrib["name"]
    struct = StructureType(struct_name)

    if "structextends" in type.attrib:
        extends_list = type.attrib["structextends"].split(",")
        struct.extends = extends_list

    for member in type.findall("./member"):
        entity = parse_typed_entity(stringify_tag_except_comment(member))

        optional = False
        if "optional" in member.attrib and member.attrib["optional"].find("true") != -1:
            optional = True

        default_value = None
        if "values" in member.attrib:
            default_value = member.attrib["values"]

        length = None
        if "len" in member.attrib:
            length_value = member.attrib["len"].split(",")[0]
            for member_name in type.findall("./member/name"):
                if member_name.text == length_value:
                    length = length_value
            if length_value == "null-terminated":
                length = "null-terminated"

        if length == None and "altlen" in member.attrib:
            length = member.attrib["altlen"]

        struct.add_member(entity, optional, default_value, length)

    STRUCTURE_TYPES[struct_name] = struct

def parse_union(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = AliasType(type.attrib["name"], type.attrib["alias"])
        return

    union_name = type.attrib["name"]
    union = UnionType(union_name)

    for member in type.findall("./member"):
        entity = parse_typed_entity(stringify_tag_except_comment(member))
        union.add_member(entity)

    UNION_TYPES[union_name] = union

def parse_constant(tag) -> Entity:
    if "alias" in tag.attrib:
        CONSTANTS[tag.attrib["name"]] = CONSTANTS[tag.attrib["alias"]]
        return CONSTANTS[tag.attrib["name"]]

    name = tag.attrib["name"]
    value = tag.attrib["value"]

    if value.startswith("(") and value.endswith(")"):
        value = value[1:-1]

    if value.endswith("f"):
        float_value = float(value[:-1])
        CONSTANTS[name] = RealConstant(name, float_value, 32)
        return CONSTANTS[name]
    if value.startswith("\"") and value.endswith("\""):
        str_value = value[1:-1]
        CONSTANTS[name] = StringConstant(name, str_value)
        return CONSTANTS[name]
    else:
        # All of this is quite shitty, but you know...
        minus_index = value.find("-")
        offset = 0
        if minus_index != -1:
            offset = -int(value[minus_index + 1:])
            value = value[:minus_index]

        size = 32
        if value.endswith("U"):
            size = 32
            value = value[:-1]
        elif value.endswith("ULL"):
            size = 64
            value = value[:-3]

        int_value = 0
        if value == "~0":
            int_value = (1 << size) - 1
        else:
            int_value = int(value, 0)

        int_value += offset

        CONSTANTS[name] = IntegerConstant(name, int_value, size)
        return CONSTANTS[name]

def parse_funcpointer(type):
    s = stringify_tag_except_comment(type)
    tokens = TokenString(s)

    if not tokens.maybe_eat_token("typedef"):
        raise Exception("Not typedef in function pointer? What?")

    return_type = parse_type_reference(tokens)
    prototype = FunctionPrototype(return_type)

    if not tokens.maybe_eat_tokens(["(", "VKAPI_PTR", "*"]):
        raise Exception("Bad funcpointer format")

    name = tokens.eat_next_identifier()

    if not tokens.maybe_eat_tokens([")", "("]) or not tokens.maybe_eat_tokens_end([")", ";"]):
        raise Exception("Bad funcpointer format")

    if tokens.s != "void":
        arguments = [s.strip() for s in tokens.s.split(",")]
        for arg in arguments:
            entity = parse_typed_entity(arg)
            prototype.add_argument(entity, False, None)

    FUNCTION_POINTER_TYPES[name] = FunctionPointerType(name, prototype)

# @Todo Parse success codes
# @Todo Parse error codes

def parse_command(tag):
    if "alias" in tag.attrib:
        COMMANDS[tag.attrib["name"]] = COMMANDS[tag.attrib["alias"]]
        return

    proto_tag = tag.find("./proto")
    proto_str = stringify_tag_except_comment(proto_tag)
    proto_tokens = TokenString(proto_str)
    return_type = parse_type_reference(proto_tokens)
    name = proto_tokens.eat_next_identifier()

    prototype = FunctionPrototype(return_type)

    if not proto_tokens.is_finished():
        raise Exception("Invalid proto format")

    for param_tag in tag.findall("./param"):
        optional = False
        if "optional" in param_tag.attrib and param_tag.attrib["optional"].find("true") != -1:
            optional = True

        length = None
        if "len" in param_tag.attrib:
            length = param_tag.attrib["len"]

        param_str = stringify_tag_except_comment(param_tag)
        param = parse_typed_entity(param_str)
        prototype.add_argument(param, optional, length)

    COMMANDS[name] = Command(name, prototype)

def parse_feature(tag):
    FEATURES[tag.attrib["name"]] = Feature(tag.attrib["name"], tag.attrib["number"])

def parse_extension(tag):
    if "supported" in tag.attrib and tag.attrib["supported"] == "disabled":
        return

    name = tag.attrib["name"]
    number = int(tag.attrib["number"])
    type = tag.attrib["type"]
    deps = []

    cmd_type = CMD_TYPE_INSTANCE
    if type == "device":
        cmd_type = CMD_TYPE_DEVICE

    if "requires" in tag.attrib:
        deps = tag.attrib["requires"].split(",")

    EXTENSIONS[name] = Extension(name, number, deps, cmd_type)

def parse_define(tag):
    name = None
    if "name" in tag.attrib:
        name = tag.attrib["name"]
    else:
        name = tag.find("./name").text

    if name:
        EMPTY[name] = Entity(name)

def parse_include(tag):
    if "name" in tag.attrib:
        name = tag.attrib["name"]
        EMPTY[name] = Entity(name)

def parse_misc_type(tag):
    name = tag.attrib["name"]
    EMPTY[name] = Entity(name)

for type in ROOT.findall("./types/type"):
    if "category" in type.attrib:
        category = type.attrib["category"]
        if category == "basetype":
            parse_basetype(type)
        elif category == "handle":
            parse_handle(type)
        elif category == "enum":
            parse_enum(type)
        elif category == "bitmask":
            parse_bitmask(type)
        elif category == "struct":
            parse_struct(type)
        elif category == "union":
            parse_union(type)
        elif category == "funcpointer":
            parse_funcpointer(type)
        elif category == "include":
            parse_include(type)
        elif category == "define":
            parse_define(type)
        else:
            raise Exception("Unhandled type")
    else:
        parse_misc_type(type)

for enum in ROOT.findall("./enums"):
    if "type" in enum.attrib:
        continue

    for e in enum.findall("./enum"):
        parse_constant(e)

for c in ROOT.findall("./commands/command"):
    parse_command(c)

for f in ROOT.findall("./feature"):
    parse_feature(f)

for e in ROOT.findall("./extensions/extension"):
    parse_extension(e)

version_int = version_to_int(VERSION)

selected_features: List[Feature] = []
for feature_str in FEATURES:
    if version_int >= FEATURES[feature_str].version_int:
        selected_features.append(FEATURES[feature_str])

selected_features = sorted(selected_features, key = lambda x: x.version_int)

features_to_load: List[str] = [f.name for f in selected_features]

extensions_to_visit: List[str] = WANTED_EXTENSIONS
extensions_visited: List[str] = []

while len(extensions_to_visit) > 0:
    extension_name = extensions_to_visit[-1]
    extensions_to_visit = extensions_to_visit[:-1]

    if not extension_name in extensions_visited:
        extensions_visited.append(extension_name)

        if not extension_name in EXTENSIONS:
            raise Exception("Extension %s not found" % extension_name)

        extension = EXTENSIONS[extension_name]

        for e in extension.dependencies:
            extensions_to_visit.append(e)

DEVICE_BASE_TYPES = ["VkDevice", "VkQueue", "VkCommandBuffer"]

for cmd_name in COMMANDS:
    cmd = COMMANDS[cmd_name]
    first_arg_type = cmd.prototype.arguments[0].id.type.get_base_type()
    if first_arg_type in DEVICE_BASE_TYPES:
        cmd.type = CMD_TYPE_DEVICE

COMMANDS["vkGetInstanceProcAddr"].type = CMD_TYPE_STATIC
COMMANDS["vkCreateInstance"].type = CMD_TYPE_ENTRY
COMMANDS["vkEnumerateInstanceExtensionProperties"].type = CMD_TYPE_ENTRY
COMMANDS["vkEnumerateInstanceLayerProperties"].type = CMD_TYPE_ENTRY
COMMANDS["vkDestroyInstance"].type = CMD_TYPE_INSTANCE
COMMANDS["vkGetDeviceProcAddr"].type = CMD_TYPE_INSTANCE
COMMANDS["vkDestroyInstance"].type = CMD_TYPE_INSTANCE

entities_visited: List[str] = []
entities_to_visit: List[Entity] = []

def parse_require(tag, extnumber: int, override_command_type: Optional[str]):
    if "feature" in tag.attrib:
        if not tag.attrib["feature"] in features_to_load:
            return

    if "extension" in tag.attrib:
        if not tag.attrib["extension"] in extensions_visited:
            return

    for x in r.findall("./*"):
        entity = None
        if x.tag == "type":
            entity = find_entity(x.attrib["name"])
        elif x.tag == "enum":
            if "extends" in x.attrib:
                entity = parse_enum_extends(x, extnumber)
            elif "value" in x.attrib:
                entity = parse_constant(x)
            else:
                entity = find_entity(x.attrib["name"])
        elif x.tag == "command":
            entity = find_entity(x.attrib["name"])
            if override_command_type != None:
                entity.type = override_command_type
        elif x.tag == "comment":
            continue
        else:
            raise Exception("Unknown required entity type %s" % x.tag)

        if not entity:
            raise Exception("Unknown entity %s" % x.attrib["name"])
        else:
            entities_to_visit.append(entity)

for feature_name in features_to_load:
    for r in ROOT.findall("./feature[@name='%s']/require" % feature_name):
        parse_require(r, 0, None)

for extension_name in extensions_visited:
    for r in ROOT.findall("./extensions/extension[@name='%s']/require" % extension_name):
        extension = EXTENSIONS[extension_name]
        parse_require(r, extension.number, extension.type)

while len(entities_to_visit) > 0:
    entity = entities_to_visit[-1]
    entities_to_visit = entities_to_visit[:-1]

    if not entity.name in entities_visited:
        entities_visited.append(entity.name)
        for e in entity.make_depset().deps:
            to_visit = find_entity(e)
            if not to_visit:
                raise Exception("Unknown entity %s" % e)
            else:
                entities_to_visit.append(to_visit)

dst = xml.Element("vulkan")

for entity_name in entities_visited:
    entity = find_entity(entity_name)
    entity.to_xml(dst)

dst[:] = sorted(dst, key=lambda x: x.tag)

with open(OUTPUT, "wb+") as fp:
    fp.write(minidom.parseString(xml.tostring(dst)).toprettyxml(indent="   ", encoding="utf8"))
