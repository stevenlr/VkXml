from model import *
import xml.etree.ElementTree as xml

BASE_TYPES = []
STRUCTURE_TYPES = []
UNION_TYPES = []
STRING_CONSTANTS = []
REAL_CONSTANTS = []
INTEGER_CONSTANTS = []
PRELUDE_TYPES = []
HANDLE_TYPES = []
ENUM_TYPES = []
FUNCTION_POINTER_TYPES = []
COMMANDS = []
BITMASK_TYPES = []
ALIAS_TYPES = []

def load_model(file):
    root = xml.parse(file).getroot()
    for child in root:
        if child.tag == "base":
            BASE_TYPES.append(BaseType.from_xml(child))
        elif child.tag == "struct":
            STRUCTURE_TYPES.append(StructureType.from_xml(child))
        elif child.tag == "union":
            UNION_TYPES.append(UnionType.from_xml(child))
        elif child.tag == "string-constant":
            STRING_CONSTANTS.append(StringConstant.from_xml(child))
        elif child.tag == "real-constant":
            REAL_CONSTANTS.append(RealConstant.from_xml(child))
        elif child.tag == "integer-constant":
            INTEGER_CONSTANTS.append(IntegerConstant.from_xml(child))
        elif child.tag == "prelude-type":
            PRELUDE_TYPES.append(Entity.from_xml(child))
        elif child.tag == "handle":
            HANDLE_TYPES.append(HandleType.from_xml(child))
        elif child.tag == "enum":
            ENUM_TYPES.append(EnumType.from_xml(child))
        elif child.tag == "function-pointer":
            FUNCTION_POINTER_TYPES.append(FunctionPointerType.from_xml(child))
        elif child.tag == "command":
            COMMANDS.append(Command.from_xml(child))
        elif child.tag == "bitmask":
            BITMASK_TYPES.append(BitmaskType.from_xml(child))
        elif child.tag == "alias":
            ALIAS_TYPES.append(AliasType.from_xml(child))
        else:
            raise Exception("Forgot about something? %s" % child.tag)

load_model("output.xml")
print(0)