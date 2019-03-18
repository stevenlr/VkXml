from xml.etree import ElementTree
import xml.etree

ROOT = ElementTree.parse("vk.xml").getroot()
BASE_TYPES = {}
HANDLE_TYPES = {}
ALIAS_TYPES = {}
ENUM_TYPES = {}
BITMASK_TYPES = {}
STRUCTURE_TYPES = {}
UNION_TYPES = {}

def parse_basetype(type):
    name = type.find("./name").text
    base = type.find("./type").text
    BASE_TYPES[name] = base

def parse_handle(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = type.attrib["alias"]
    else:
        handle_type = type.find("./type").text
        name = type.find("./name").text
        if handle_type == "VK_DEFINE_HANDLE":
            HANDLE_TYPES[name] = "uintptr_t"
        elif handle_type == "VK_DEFINE_NON_DISPATCHABLE_HANDLE":
            HANDLE_TYPES[name] = "uint64_t"
        else:
            print("Unknown handle type %s for %s" % (handle_type, name))

def parse_enum(type):
    name = type.attrib["name"]
    if "alias" in type.attrib:
        ALIAS_TYPES[name] = type.attrib["alias"]
    else:
        enums = ROOT.find("./enums[@name='%s']" % name)
        if enums == None:
            ENUM_TYPES[name] = {}
        else:
            enum_type = enums.attrib["type"]
            values = {}
            for e in enums.findall("./enum"):
                if "alias" in e.attrib:
                    values[e.attrib["name"]] = values[e.attrib["alias"]]
                elif "bitpos" in e.attrib:
                    values[e.attrib["name"]] = str(1 << int(e.attrib["bitpos"]))
                else:
                    values[e.attrib["name"]] = e.attrib["value"]
            ENUM_TYPES[name] = values

def parse_bitmask(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = type.attrib["alias"]
    else:
        name = type.find("./name").text
        if "requires" in type.attrib:
            BITMASK_TYPES[name] = type.attrib["requires"]
        else:
            BITMASK_TYPES[name] = None

def element_tokens(element):
    t = []
    if element.text != None and element.text.strip() != "":
        t.append(element.text.strip())
    for e in element:
        t.append(e)
        if e.tail != None and e.tail.strip() != "":
            t.append(e.tail.strip())
    return t

def parse_typed_identifier(tokens):
    is_const = False
    if not isinstance(tokens[0], ElementTree.Element):
        if tokens[0] == "const":
            is_const = True
        elif tokens[0] == "const struct":
            pass
        elif tokens[0] == "struct":
            pass
        else:
            print("Unknown token " + tokens[0])
            return None
        tokens = tokens[1:]

    type_name = None
    if tokens[0].tag == "type":
        type_name = tokens[0].text
    else:
        print("Unknown member structure")
        return None
    tokens = tokens[1:]

    is_pointer = False
    is_pointer_const_pointer = False
    if not isinstance(tokens[0], ElementTree.Element):
        if tokens[0] == "*":
            is_pointer = True
        elif tokens[0] == "* const*":
            is_pointer_const_pointer = True
        else:
            print("Unknown token " + tokens[0])
            return None
        tokens = tokens[1:]

    field_name = None
    if tokens[0].tag == "name":
        field_name = tokens[0].text
    else:
        print("Unknown member structure")
        return None
    tokens = tokens[1:]

    is_array = False
    array_count = None
    if len(tokens) > 0 and isinstance(tokens[0], str) and tokens[0].startswith("["):
        is_array = True

        if tokens[0].endswith("]"):
            array_count = tokens[0][1:-1]
            tokens = tokens[1:]
        else:
            if isinstance(tokens[1], ElementTree.Element) and tokens[1].tag == "enum":
                array_count = tokens[1].text
            else:
                print("Invalid array length qualifier")
                return None

            if not isinstance(tokens[2], str) or not tokens[2] == "]":
                print("Invalid array length qualifier")
                return None
            tokens = tokens[3:]

    if len(tokens) > 0 and isinstance(tokens[0], ElementTree.Element) and tokens[0].tag == "comment":
        tokens = tokens[1:]

    if len(tokens) > 0:
        print("Unknown member structure")
        return None

    return {
        "field_name": field_name,
        "is_const": is_const,
        "is_pointer": is_pointer,
        "is_pointer_const_pointer": is_pointer_const_pointer,
        "type_name": type_name,
        "is_array": is_array,
        "array_count": array_count,
    }

def parse_struct(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = type.attrib["alias"]
        return

    members = []
    for member in type.findall("./member"):
        tokens = element_tokens(member)
        m = parse_typed_identifier(tokens)
        if m != None:
            members.append(m)
    STRUCTURE_TYPES[type.attrib["name"]] = members

def parse_union(type):
    if "alias" in type.attrib:
        ALIAS_TYPES[type.attrib["name"]] = type.attrib["alias"]
        return

    members = []
    for member in type.findall("./member"):
        tokens = element_tokens(member)
        m = parse_typed_identifier(tokens)
        if m != None:
            members.append(m)
    UNION_TYPES[type.attrib["name"]] = members

def parse_funcpointer(type):
    tokens = element_tokens(type)
    if tokens[0].startswith("typedef") and tokens[0].endswith("(VKAPI_PTR *"):
        return_type_str = tokens[0][7:-12].strip()
        return_is_const = False
        return_type_name = None
        return_is_pointer = False

        if return_type_str.startswith("const "):
            return_is_const = True
            return_type_str = return_type_str[5:].strip()

        if return_type_str.endswith("*"):
            return_is_pointer = True
            return_type_str = return_type_str[:-1].strip()

        return_type_name = return_type_str

        return_type = {
            "is_const": return_is_const,
            "is_pointer": return_is_pointer,
            "type_name": return_type_name,
        }

        name = None
        if tokens[1].tag == "name":
            name = tokens[1].text
        tokens = tokens[2:]

        args = []
        if len(tokens) > 1:
            pass

        print(name, args)
    else:
        print("Bad funcpointer format")
        return

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
        else:
            print("Unknown type category %s" % category)
            pass
    else:
        # print("Type with no category")
        pass
