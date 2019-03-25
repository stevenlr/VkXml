from typing import *
import re
from xml.etree import ElementTree
import xml.etree

class DependenciesSet:
    def __init__(self):
        self.deps: List[str] = []

    def add_dep(self, dep: str):
        if dep != None and dep not in self.deps:
            self.deps.append(dep)

class Entity:
    def __init__(self, name: str):
        self.name = name

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        return depset

class Type:
    def get_base_type(self):
        return None

class TypeReference(Type):
    def __init__(self, name: str, is_const: bool):
        self.name = name
        self.is_const = is_const

    def get_base_type(self):
        return self.name

class PointerType(Type):
    def __init__(self, inner: Type, is_const: bool):
        self.inner = inner
        self.is_const = is_const

    def get_base_type(self):
        return self.inner.get_base_type()

class ArrayType(Type):
    def __init__(self, inner: Type, length: int):
        self.inner = inner
        self.length = length

    def get_base_type(self):
        return self.inner.get_base_type()

class TypedIdentifier:
    def __init__(self, name: str, type: Type):
        self.name = name
        self.type = type

class BaseType(Entity):
    def __init__(self, name: str, alias: str):
        Entity.__init__(self, name)
        self.alias = alias

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.alias)
        return depset

class AliasType(Entity):
    def __init__(self, name: str, alias: str):
        Entity.__init__(self, name)
        self.alias = alias

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.alias)
        return depset

class BitmaskType(Entity):
    def __init__(self, name: str, alias: str):
        Entity.__init__(self, name)
        self.alias = alias

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.alias)
        return depset

class HandleType(Entity):
    def __init__(self, name: str, dispatchable: bool):
        Entity.__init__(self, name)
        self.dispatchable = dispatchable

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        if self.dispatchable:
            depset.add_dep("uintptr_t")
        else:
            depset.add_dep("uint64_t")
        return depset

class EnumType(Entity):
    def __init__(self, name: str):
        Entity.__init__(self, name)
        self.values: Dict[str, int] = {}

    def add_value(self, name: str, value: int):
        self.values[name] = value

    def get_value(self, name: str) -> int:
        if not name in self.values:
            raise Exception("Unknown enum value")
        return self.values[name]

class StructureType(Entity):
    def __init__(self, name: str):
        Entity.__init__(self, name)
        self.members: List[TypedIdentifier] = []

    def add_member(self, entity: TypedIdentifier):
        self.members.append(entity)

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        for member in self.members:
            depset.add_dep(member.type.get_base_type())
        return depset

class UnionType(Entity):
    def __init__(self, name: str):
        Entity.__init__(self, name)
        self.members: List[TypedIdentifier] = []

    def add_member(self, entity: TypedIdentifier):
        self.members.append(entity)

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        for member in self.members:
            depset.add_dep(member.type.get_base_type())
        return depset

class FunctionPrototype:
    def __init__(self, return_type: Type):
        self.return_type = return_type
        self.arguments: List[TypedIdentifier] = []

    def add_argument(self, arg: TypedIdentifier):
        self.arguments.append(arg)

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.return_type.get_base_type())
        for member in self.arguments:
            depset.add_dep(member.type.get_base_type())
        return depset

class Constant(Entity):
    def __init__(self, name: str):
        Entity.__init__(self, name)

class IntegerConstant(Constant):
    def __init__(self, name: str, value: int, size: int):
        Constant.__init__(self, name)
        self.value = value
        self.size = size

class RealConstant(Constant):
    def __init__(self, name: str, value: float, size: int):
        Constant.__init__(self, name)
        self.value = value
        self.size = size

class FunctionPointerType(Entity):
    def __init__(self, name: str, prototype: FunctionPrototype):
        Entity.__init__(self, name)
        self.prototype = prototype

    def make_depset(self) -> DependenciesSet:
        return self.prototype.make_depset()

class Command(Entity):
    def __init__(self, name: str, prototype: FunctionPrototype):
        Entity.__init__(self, name)
        self.prototype = prototype

    def make_depset(self) -> DependenciesSet:
        return self.prototype.make_depset()

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
    def __init__(self, name: str, number: int, dependencies: List[str]):
        self.name = name
        self.number = number
        self.dependencies = dependencies

ROOT = ElementTree.parse("vk.xml").getroot()
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
        enum = EnumType(name)
        if enums != None:
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

# @Todo Parse length information
# @Todo Parse optional values
# @Todo Parse default values

def parse_struct(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = AliasType(type.attrib["name"], type.attrib["alias"])
        return

    struct_name = type.attrib["name"]
    struct = StructureType(struct_name)

    for member in type.findall("./member"):
        entity = parse_typed_entity(stringify_tag_except_comment(member))
        struct.add_member(entity)

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

def parse_constant(tag):
    if "alias" in tag.attrib:
        CONSTANTS[tag.attrib["name"]] = CONSTANTS[tag.attrib["alias"]]
        return

    name = tag.attrib["name"]
    value = tag.attrib["value"]

    if value.startswith("(") and value.endswith(")"):
        value = value[1:-1]

    if value.endswith("f"):
        float_value = float(value[:-1])
        CONSTANTS[name] = RealConstant(name, float_value, 32)
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
            prototype.add_argument(entity)

    FUNCTION_POINTER_TYPES[name] = FunctionPointerType(name, prototype)

# @Todo Parse success codes
# @Todo Parse error codes
# @Todo Parse optionals
# @Todo Categorize entry points, instance & device commands

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
        param_str = stringify_tag_except_comment(param_tag)
        param = parse_typed_entity(param_str)
        prototype.add_argument(param)

    COMMANDS[name] = Command(name, prototype)

# @Todo Create required types and commands sets from required features

def parse_feature(tag):
    FEATURES[tag.attrib["name"]] = Feature(tag.attrib["name"], tag.attrib["number"])

def parse_extension(tag):
    name = tag.attrib["name"]
    number = int(tag.attrib["number"])
    deps = []

    if "requires" in tag.attrib:
        deps = tag.attrib["requires"].split(",")

    EXTENSIONS[name] = Extension(name, number, deps)

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
        pass

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

VERSION = "1.0"

version_int = version_to_int(VERSION)

selected_features: List[Feature] = []
for feature_str in FEATURES:
    if version_int >= FEATURES[feature_str].version_int:
        selected_features.append(FEATURES[feature_str])

selected_features = sorted(selected_features, key = lambda x: x.version_int)

FEATURES_TO_LOAD: List[str] = [f.name for f in selected_features]

entities_visited: List[str] = []
entities_to_visit: List[Entity] = []

for feature_name in FEATURES_TO_LOAD:
    for r in ROOT.findall("./feature[@name='%s']/require" % feature_name):
        # @Todo Handle dependencies
        # @Todo Handle conditionals
        for x in r.findall("./*"):
            entity = None
            if x.tag == "type":
                entity = find_entity(x.attrib["name"])
            elif x.tag == "enum":
                entity = find_entity(x.attrib["name"])
            elif x.tag == "command":
                entity = find_entity(x.attrib["name"])
            else:
                raise Exception("Unknown required entity type %s", x.tag)

            if not entity:
                raise Exception("Unknown entity %s" % x.attrib["name"])
            else:
                entities_to_visit.append(entity)

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
