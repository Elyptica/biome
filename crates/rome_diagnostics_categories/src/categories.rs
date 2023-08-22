// This file contains the list of all diagnostic categories for the Biome
// toolchain
//
// The `define_categories` macro is preprocessed in the build script for the
// crate in order to generate the static registry. The body of the macro
// consists of a list of key-value pairs defining the categories that have an
// associated hyperlink, then a list of string literals defining the remaining
// categories without a link.

define_categories! {
    // Lint categories
    // a11y
    "lint/a11y/noAccessKey": "https://biomejs.dev/lint/rules/noAccessKey",
    "lint/a11y/noAutofocus": "https://biomejs.dev/lint/rules/noAutofocus",
    "lint/a11y/noBlankTarget": "https://biomejs.dev/lint/rules/noBlankTarget",
    "lint/a11y/noDistractingElements": "https://biomejs.dev/lint/rules/noDistractingElements",
    "lint/a11y/noHeaderScope": "https://biomejs.dev/lint/rules/noHeaderScope",
    "lint/a11y/noNoninteractiveElementToInteractiveRole": "https://biomejs.dev/lint/rules/noNoninteractiveElementToInteractiveRole",
    "lint/a11y/noPositiveTabindex": "https://biomejs.dev/lint/rules/noPositiveTabindex",
    "lint/a11y/noRedundantAlt": "https://biomejs.dev/lint/rules/noRedundantAlt",
    "lint/a11y/noSvgWithoutTitle": "https://biomejs.dev/lint/rules/noSvgWithoutTitle",
    "lint/a11y/useAltText": "https://biomejs.dev/lint/rules/useAltText",
    "lint/a11y/useAnchorContent": "https://biomejs.dev/lint/rules/useAnchorContent",
    "lint/a11y/useAriaPropsForRole": "https://biomejs.dev/lint/rules/useAriaPropsForRole",
    "lint/a11y/useButtonType": "https://biomejs.dev/lint/rules/useButtonType",
    "lint/a11y/useHeadingContent": "https://biomejs.dev/lint/rules/useHeadingContent",
    "lint/a11y/useHtmlLang": "https://biomejs.dev/lint/rules/useHtmlLang",
    "lint/a11y/useIframeTitle": "https://biomejs.dev/lint/rules/useIframeTitle",
    "lint/a11y/useKeyWithClickEvents": "https://biomejs.dev/lint/rules/useKeyWithClickEvents",
    "lint/a11y/useKeyWithMouseEvents": "https://biomejs.dev/lint/rules/useKeyWithMouseEvents",
    "lint/a11y/useMediaCaption": "https://biomejs.dev/lint/rules/useMediaCaption",
    "lint/a11y/useValidAnchor": "https://biomejs.dev/lint/rules/useValidAnchor",
    "lint/a11y/useValidAriaProps":"https://biomejs.dev/lint/rules/useValidAriaProps",
    "lint/a11y/useValidLang":"https://biomejs.dev/lint/rules/useValidLang",

    // complexity
    "lint/complexity/noExtraBooleanCast": "https://biomejs.dev/lint/rules/noExtraBooleanCast",
    "lint/complexity/noForEach": "https://biomejs.dev/lint/rules/noForEach",
    "lint/complexity/noMultipleSpacesInRegularExpressionLiterals": "https://biomejs.dev/lint/rules/noMultipleSpacesInRegularExpressionLiterals",
    "lint/complexity/noUselessCatch": "https://biomejs.dev/lint/rules/noUselessCatch",
    "lint/complexity/noUselessConstructor": "https://biomejs.dev/lint/rules/noUselessConstructor",
    "lint/complexity/noUselessFragments": "https://biomejs.dev/lint/rules/noUselessFragments",
    "lint/complexity/noUselessLabel":"https://biomejs.dev/lint/rules/noUselessLabel",
    "lint/complexity/noUselessRename": "https://biomejs.dev/lint/rules/noUselessRename",
    "lint/complexity/noUselessSwitchCase": "https://biomejs.dev/lint/rules/noUselessSwitchCase",
    "lint/complexity/noUselessTypeConstraint": "https://biomejs.dev/lint/rules/noUselessTypeConstraint",
    "lint/complexity/noWith": "https://biomejs.dev/lint/rules/noWith",
    "lint/complexity/useFlatMap": "https://biomejs.dev/lint/rules/useFlatMap",
    "lint/complexity/useLiteralKeys": "https://biomejs.dev/lint/rules/useLiteralKeys",
    "lint/complexity/useOptionalChain": "https://biomejs.dev/lint/rules/useOptionalChain",
    "lint/complexity/useSimpleNumberKeys": "https://biomejs.dev/lint/rules/useSimpleNumberKeys",
    "lint/complexity/useSimplifiedLogicExpression": "https://biomejs.dev/lint/rules/useSimplifiedLogicExpression",

    // correctness
    "lint/correctness/noChildrenProp": "https://biomejs.dev/lint/rules/noChildrenProp",
    "lint/correctness/noConstAssign": "https://biomejs.dev/lint/rules/noConstAssign",
    "lint/correctness/noConstructorReturn": "https://biomejs.dev/lint/rules/noConstructorReturn",
    "lint/correctness/noEmptyPattern": "https://biomejs.dev/lint/rules/noEmptyPattern",
    "lint/correctness/noGlobalObjectCalls": "https://biomejs.dev/lint/rules/noGlobalObjectCalls",
    "lint/correctness/noInnerDeclarations": "https://biomejs.dev/lint/rules/noInnerDeclarations",
    "lint/correctness/noInvalidConstructorSuper": "https://biomejs.dev/lint/rules/noInvalidConstructorSuper",
    "lint/correctness/useIsNan": "https://biomejs.dev/lint/rules/useIsNan",
    "lint/correctness/noNewSymbol": "https://biomejs.dev/lint/rules/noNewSymbol",
    "lint/correctness/noPrecisionLoss": "https://biomejs.dev/lint/rules/noPrecisionLoss",
    "lint/correctness/noRenderReturnValue": "https://biomejs.dev/lint/rules/noRenderReturnValue",
    "lint/correctness/noSetterReturn": "https://biomejs.dev/lint/rules/noSetterReturn",
    "lint/correctness/noStringCaseMismatch": "https://biomejs.dev/lint/rules/noStringCaseMismatch",
    "lint/correctness/noSwitchDeclarations": "https://biomejs.dev/lint/rules/noSwitchDeclarations",
    "lint/correctness/noUndeclaredVariables": "https://biomejs.dev/lint/rules/noUndeclaredVariables",
    "lint/correctness/noUnnecessaryContinue": "https://biomejs.dev/lint/rules/noUnnecessaryContinue",
    "lint/correctness/noUnreachable": "https://biomejs.dev/lint/rules/noUnreachable",
    "lint/correctness/noUnreachableSuper": "https://biomejs.dev/docs/lint/rules/noUnreachableSuper",
    "lint/correctness/noUnsafeFinally": "https://biomejs.dev/lint/rules/noUnsafeFinally",
    "lint/correctness/noUnsafeOptionalChaining": "https://biomejs.dev/lint/rules/noUnsafeOptionalChaining",
    "lint/correctness/noUnusedLabels": "https://biomejs.dev/lint/rules/noUnusedLabels",
    "lint/correctness/noUnusedVariables": "https://biomejs.dev/lint/rules/noUnusedVariables",
    "lint/correctness/noVoidElementsWithChildren": "https://biomejs.dev/lint/rules/noVoidElementsWithChildren",
    "lint/correctness/noVoidTypeReturn": "https://biomejs.dev/lint/rules/noVoidTypeReturn",
    "lint/correctness/useValidForDirection": "https://biomejs.dev/lint/rules/useValidForDirection",
    "lint/correctness/useYield": "https://biomejs.dev/lint/rules/useYield",

    // nursery
    "lint/nursery/noAccumulatingSpread": "https://biomejs.dev/lint/rules/noAccumulatingSpread",
    "lint/nursery/noAriaUnsupportedElements": "https://biomejs.dev/lint/rules/noAriaUnsupportedElements",
    "lint/nursery/noBannedTypes":"https://biomejs.dev/lint/rules/noBannedTypes",
    "lint/nursery/noConfusingArrow": "https://biomejs.dev/lint/rules/noConfusingArrow",
    "lint/nursery/noConstantCondition": "https://biomejs.dev/lint/rules/noConstantCondition",
    "lint/nursery/noControlCharactersInRegex": "https://biomejs.dev/lint/rules/noControlCharactersInRegex",
    "lint/nursery/noDuplicateJsonKeys": "https://biomejs.dev/lint/rules/noDuplicateJsonKeys",
    "lint/nursery/noExcessiveComplexity": "https://biomejs.dev/lint/rules/noExcessiveComplexity",
    "lint/nursery/noFallthroughSwitchClause": "https://biomejs.dev/lint/rules/noFallthroughSwitchClause",
    "lint/nursery/noGlobalIsFinite": "https://biomejs.dev/lint/rules/noGlobalIsFinite",
    "lint/nursery/noGlobalIsNan": "https://biomejs.dev/lint/rules/noGlobalIsNan",
    "lint/nursery/noNoninteractiveTabindex": "https://biomejs.dev/lint/rules/noNoninteractiveTabindex",
    "lint/nursery/noNonoctalDecimalEscape": "https://biomejs.dev/lint/rules/noNonoctalDecimalEscape",
    "lint/nursery/noRedundantRoles": "https://biomejs.dev/lint/rules/noRedundantRoles",
    "lint/nursery/noSelfAssign": "https://biomejs.dev/lint/rules/noSelfAssign",
    "lint/nursery/noStaticOnlyClass": "https://biomejs.dev/lint/rules/noStaticOnlyClass",
    "lint/nursery/noUnsafeDeclarationMerging": "https://biomejs.dev/lint/rules/noUnsafeDeclarationMerging",
    "lint/nursery/noUselessEmptyExport": "https://biomejs.dev/lint/rules/noUselessEmptyExport",
    "lint/nursery/noUselessThisAlias": "https://biomejs.dev/lint/rules/noUselessThisAlias",
    "lint/nursery/noVoid": "https://biomejs.dev/lint/rules/noVoid",
    "lint/nursery/useAriaPropTypes": "https://biomejs.dev/lint/rules/useAriaPropTypes",
    "lint/nursery/useArrowFunction": "https://biomejs.dev/lint/rules/useArrowFunction",
    "lint/nursery/useExhaustiveDependencies": "https://biomejs.dev/lint/rules/useExhaustiveDependencies",
    "lint/nursery/useGetterReturn": "https://biomejs.dev/lint/rules/useGetterReturn",
    "lint/nursery/useGroupedTypeImport": "https://biomejs.dev/lint/rules/useGroupedTypeImport",
    "lint/nursery/useHookAtTopLevel": "https://biomejs.dev/lint/rules/useHookAtTopLevel",
    "lint/nursery/useImportRestrictions": "https://biomejs.dev/lint/rules/useImportRestrictions",
    "lint/nursery/useIsArray": "https://biomejs.dev/lint/rules/useIsArray",
    "lint/nursery/useLiteralEnumMembers": "https://biomejs.dev/lint/rules/useLiteralEnumMembers",
    "lint/nursery/useNamingConvention": "https://biomejs.dev/lint/rules/useNamingConvention",
    // nursery end

    // performance
    "lint/performance/noDelete": "https://biomejs.dev/lint/rules/noDelete",

    // security
    "lint/security/noDangerouslySetInnerHtml": "https://biomejs.dev/lint/rules/noDangerouslySetInnerHtml",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://biomejs.dev/lint/rules/noDangerouslySetInnerHtmlWithChildren",

    // style
    "lint/style/noArguments": "https://biomejs.dev/lint/rules/noArguments",
    "lint/style/noCommaOperator": "https://biomejs.dev/lint/rules/noCommaOperator",
    "lint/style/noImplicitBoolean": "https://biomejs.dev/lint/rules/noImplicitBoolean",
    "lint/style/noInferrableTypes": "https://biomejs.dev/lint/rules/noInferrableTypes",
    "lint/style/noNamespace": "https://biomejs.dev/lint/rules/noNamespace",
    "lint/style/noNegationElse": "https://biomejs.dev/lint/rules/noNegationElse",
    "lint/style/noNonNullAssertion": "https://biomejs.dev/lint/rules/noNonNullAssertion",
    "lint/style/noParameterAssign": "https://biomejs.dev/lint/rules/noParameterAssign",
    "lint/style/noParameterProperties": "https://biomejs.dev/lint/rules/noParameterProperties",
    "lint/style/noRestrictedGlobals": "https://biomejs.dev/lint/rules/noRestrictedGlobals",
    "lint/style/noShoutyConstants": "https://biomejs.dev/lint/rules/noShoutyConstants",
    "lint/style/noUnusedTemplateLiteral": "https://biomejs.dev/lint/rules/noUnusedTemplateLiteral",
    "lint/style/noVar": "https://biomejs.dev/lint/rules/noVar",
    "lint/style/useBlockStatements": "https://biomejs.dev/lint/rules/useBlockStatements",
    "lint/style/useConst":"https://biomejs.dev/lint/rules/useConst",
    "lint/style/useDefaultParameterLast":"https://biomejs.dev/lint/rules/useDefaultParameterLast",
    "lint/style/useEnumInitializers":"https://biomejs.dev/lint/rules/useEnumInitializers",
    "lint/style/useExponentiationOperator": "https://biomejs.dev/lint/rules/useExponentiationOperator",
    "lint/style/useFragmentSyntax": "https://biomejs.dev/lint/rules/useFragmentSyntax",
    "lint/style/useNumericLiterals": "https://biomejs.dev/lint/rules/useNumericLiterals",
    "lint/style/useSelfClosingElements": "https://biomejs.dev/lint/rules/useSelfClosingElements",
    "lint/style/useShorthandArrayType": "https://biomejs.dev/lint/rules/useShorthandArrayType",
    "lint/style/useSingleCaseStatement": "https://biomejs.dev/lint/rules/useSingleCaseStatement",
    "lint/style/useSingleVarDeclarator": "https://biomejs.dev/lint/rules/useSingleVarDeclarator",
    "lint/style/useTemplate": "https://biomejs.dev/lint/rules/useTemplate",
    "lint/style/useWhile": "https://biomejs.dev/lint/rules/useWhile",

    // suspicious
    "lint/suspicious/noArrayIndexKey": "https://biomejs.dev/lint/rules/noArrayIndexKey",
    "lint/suspicious/noAssignInExpressions": "https://biomejs.dev/lint/rules/noAssignInExpressions",
    "lint/suspicious/noAsyncPromiseExecutor": "https://biomejs.dev/lint/rules/noAsyncPromiseExecutor",
    "lint/suspicious/noCatchAssign": "https://biomejs.dev/lint/rules/noCatchAssign",
    "lint/suspicious/noClassAssign": "https://biomejs.dev/lint/rules/noClassAssign",
    "lint/suspicious/noCommentText": "https://biomejs.dev/lint/rules/noCommentText",
    "lint/suspicious/noCompareNegZero": "https://biomejs.dev/lint/rules/noCompareNegZero",
    "lint/suspicious/noConfusingLabels": "https://biomejs.dev/lint/rules/noConfusingLabels",
    "lint/suspicious/noConsoleLog": "https://biomejs.dev/lint/rules/noConsoleLog",
    "lint/suspicious/noConstEnum": "https://biomejs.dev/lint/rules/noConstEnum",
    "lint/suspicious/noDebugger": "https://biomejs.dev/lint/rules/noDebugger",
    "lint/suspicious/noDoubleEquals": "https://biomejs.dev/lint/rules/noDoubleEquals",
    "lint/suspicious/noDuplicateCase": "https://biomejs.dev/lint/rules/noDuplicateCase",
    "lint/suspicious/noDuplicateClassMembers": "https://biomejs.dev/lint/rules/noDuplicateClassMembers",
    "lint/suspicious/noDuplicateJsxProps": "https://biomejs.dev/lint/rules/noDuplicateJsxProps",
    "lint/suspicious/noDuplicateObjectKeys":"https://biomejs.dev/lint/rules/noDuplicateObjectKeys",
    "lint/suspicious/noDuplicateParameters": "https://biomejs.dev/lint/rules/noDuplicateParameters",
    "lint/suspicious/noEmptyInterface": "https://biomejs.dev/lint/rules/noEmptyInterface",
    "lint/suspicious/noExplicitAny": "https://biomejs.dev/lint/rules/noExplicitAny",
    "lint/suspicious/noExtraNonNullAssertion":"https://biomejs.dev/lint/rules/noExtraNonNullAssertion",
    "lint/suspicious/noFunctionAssign": "https://biomejs.dev/lint/rules/noFunctionAssign",
    "lint/suspicious/noImportAssign": "https://biomejs.dev/lint/rules/noImportAssign",
    "lint/suspicious/noLabelVar": "https://biomejs.dev/lint/rules/noLabelVar",
    "lint/suspicious/noPrototypeBuiltins": "https://biomejs.dev/lint/rules/noPrototypeBuiltins",
    "lint/suspicious/noRedeclare": "https://biomejs.dev/lint/rules/noRedeclare",
    "lint/suspicious/noRedundantUseStrict": "https://biomejs.dev/lint/rules/noRedundantUseStrict",
    "lint/suspicious/noSelfCompare": "https://biomejs.dev/lint/rules/noSelfCompare",
    "lint/suspicious/noShadowRestrictedNames": "https://biomejs.dev/lint/rules/noShadowRestrictedNames",
    "lint/suspicious/noSparseArray": "https://biomejs.dev/lint/rules/noSparseArray",
    "lint/suspicious/noUnsafeNegation": "https://biomejs.dev/lint/rules/noUnsafeNegation",
    "lint/suspicious/useDefaultSwitchClauseLast":"https://biomejs.dev/lint/rules/useDefaultSwitchClauseLast",
    "lint/suspicious/useNamespaceKeyword": "https://biomejs.dev/lint/rules/useNamespaceKeyword",
    "lint/suspicious/useValidTypeof": "https://biomejs.dev/lint/rules/useValidTypeof",

    ;

    // General categories
    "files/missingHandler",
    "format",
    "check",
    "ci",
    "configuration",
    "organizeImports",
    "migrate",
    "deserialize",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",
    // parse categories
    "parse",
    "parse/noSuperWithoutExtends",
    "parse/noDuplicatePrivateClassMembers",

    // Lint groups
    "lint",
    "lint/a11y",
    "lint/complexity",
    "lint/correctness",
    "lint/nursery",
    "lint/performance",
    "lint/security",
    "lint/style",
    "lint/suspicious",

    // Suppression comments
    "suppressions/parse",
    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    "suppressions/unused",
    "suppressions/deprecatedSyntax",

    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
