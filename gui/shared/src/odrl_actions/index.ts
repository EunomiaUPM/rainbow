export const odrlActions = [
    "read",
    "analyze",
    "share",
    "ole", // OLE no es una acción ODRL estándar, la mantengo si es tuya específica.
    "use",
    "play",
    "transfer",
    "reproduce",
    "distribute",
    "display",
    "print",
    "execute",
    "modify",
    "extract",
    "transform",
    "delete",
    "install",
    "uninstall",
    "present",
    "obtain",
    "give",
    "derive",
    "aggregate",
    "disclose",
    "inform",
    "compensate",
    "attribut",
    "assign"
];

export const leftOperands = [
    "date",
    "user", // Podría ser más específico como "assignee" o "assigner"
    "location",
    "clara", // Clara no es un operando ODRL estándar, la mantengo si es tuya específica.
    "dateTime",
    "purpose",
    "recipient",
    "legalBasis",
    "language",
    "industry",
    "spatial", // para territorios
    "elapsedTime",
    "count",
    "event",
    "meter",
    "version",
    "hash"
];

export const operators = [
    "eq", // equal
    "neq", // not equal
    "gt", // greater than
    "lt", // less than
    "gteq", // greater than or equal
    "lteq", // less than or equal
    "isA", // is a type of
    "isAnyOf", // is any of the listed values
    "hasPart",
    "isPartOf",
    "isMemberOf",
    "not", // negation (often used with other operators or constraints)
    "and",
    "or",
    "xone", // exclusive or
    "andSequence"
];