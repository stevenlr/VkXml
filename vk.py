from typing import *
import re
from xml.etree import ElementTree
import xml.etree

class Type:
    pass

class TypeReference(Type):
    def __init__(self, name: str, is_const: bool):
        self.name = name
        self.is_const = is_const

class PointerType(Type):
    def __init__(self, inner: Type, is_const: bool):
        self.inner = inner
        self.is_const = is_const

class ArrayType(Type):
    def __init__(self, inner: Type, length: int):
        self.inner = inner
        self.length = length

class TypedEntity:
    def __init__(self, name: str, type: Type):
        self.name = name
        self.type = type

class BaseType:
    def __init__(self, name: str, alias: str):
        self.name = name
        self.alias = alias

class AliasType:
    def __init__(self, name: str, alias: str):
        self.name = name
        self.alias = alias

class BitmaskType:
    def __init__(self, name: str, alias: str):
        self.name = name
        self.alias = alias

class HandleType:
    def __init__(self, name: str, dispatchable: bool):
        self.name = name
        self.dispatchable = dispatchable

class EnumType:
    def __init__(self, name: str):
        self.name = name
        self.values: Dict[str, int] = {}

    def add_value(self, name: str, value: int):
        self.values[name] = value

    def get_value(self, name: str) -> int:
        if not name in self.values:
            raise Exception("Unknown enum value")
        return self.values[name]

class StructureType:
    def __init__(self, name: str):
        self.name = name
        self.members: List[TypedEntity] = []

    def add_member(self, entity: TypedEntity):
        self.members.append(entity)

class UnionType:
    def __init__(self, name: str):
        self.name = name
        self.members: List[TypedEntity] = []

    def add_member(self, entity: TypedEntity):
        self.members.append(entity)

class FunctionPrototype:
    def __init__(self, return_type: Type):
        self.return_type = return_type
        self.arguments: List[TypedEntity] = []

    def add_argument(self, arg: TypedEntity):
        self.arguments.append(arg)

class FunctionPointerType:
    def __init__(self, name: str, prototype: FunctionPrototype):
        self.name = name
        self.prototype = prototype

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
        for token in tokens:
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

ROOT = ElementTree.parse("vk.xml").getroot()
BASE_TYPES:             Dict[str, BaseType] = {}
HANDLE_TYPES:           Dict[str, HandleType] = {}
ALIAS_TYPES:            Dict[str, AliasType] = {}
ENUM_TYPES:             Dict[str, EnumType] = {}
BITMASK_TYPES:          Dict[str, BitmaskType] = {}
STRUCTURE_TYPES:        Dict[str, StructureType] = {}
UNION_TYPES:            Dict[str, UnionType] = {}
FUNCTION_POINTER_TYPES: Dict[str, FunctionPointerType] = {}

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
            BITMASK_TYPES[name] = BitmaskType(name, "")

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

def parse_typed_entity(s: str) -> TypedEntity:
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

    return TypedEntity(entity_name, type)

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

    if not tokens.maybe_eat_tokens([")", "("]) or not tokens.maybe_eat_tokens_end([";", ")"]):
        raise Exception("Bad funcpointer format")

    if tokens.s != "void":
        arguments = [s.strip() for s in tokens.s.split(",")]
        for arg in arguments:
            entity = parse_typed_entity(arg)
            prototype.add_argument(entity)

    FUNCTION_POINTER_TYPES[name] = FunctionPointerType(name, prototype)

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
            pass
        elif category == "define":
            pass
