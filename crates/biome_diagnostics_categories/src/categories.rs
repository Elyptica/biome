// This file contains the list of all diagnostic categories for the Biome
// toolchain
//
// The `define_categories` macro is preprocessed in the build script for the
// crate in order to generate the static registry. The body of the macro
// consists of a list of key-value pairs defining the categories that have an
// associated hyperlink, then a list of string literals defining the remaining
// categories without a link.

// PLEASE, DON'T EDIT THIS FILE BY HAND.
// Use `just new-lintrule` to create a new rule.
// lint rules are lexicographically sorted and
// must be between `define_categories! {\n` and `\n    ;\n`.

define_categories! {
    "lint/a11y/noAccessKey": "https://biomejs.dev/linter/rules/no-access-key",
    "lint/a11y/noAriaHiddenOnFocusable": "https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable",
    "lint/a11y/noAriaUnsupportedElements": "https://biomejs.dev/linter/rules/no-aria-unsupported-elements",
    "lint/a11y/noAutofocus": "https://biomejs.dev/linter/rules/no-autofocus",
    "lint/a11y/noBlankTarget": "https://biomejs.dev/linter/rules/no-blank-target",
    "lint/a11y/noDistractingElements": "https://biomejs.dev/linter/rules/no-distracting-elements",
    "lint/a11y/noHeaderScope": "https://biomejs.dev/linter/rules/no-header-scope",
    "lint/a11y/noInteractiveElementToNoninteractiveRole": "https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role",
    "lint/a11y/noLabelWithoutControl": "https://biomejs.dev/linter/rules/no-label-without-control",
    "lint/a11y/noNoninteractiveElementToInteractiveRole": "https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role",
    "lint/a11y/noNoninteractiveTabindex": "https://biomejs.dev/linter/rules/no-noninteractive-tabindex",
    "lint/a11y/noPositiveTabindex": "https://biomejs.dev/linter/rules/no-positive-tabindex",
    "lint/a11y/noRedundantAlt": "https://biomejs.dev/linter/rules/no-redundant-alt",
    "lint/a11y/noRedundantRoles": "https://biomejs.dev/linter/rules/no-redundant-roles",
    "lint/a11y/noSvgWithoutTitle": "https://biomejs.dev/linter/rules/no-svg-without-title",
    "lint/a11y/useAltText": "https://biomejs.dev/linter/rules/use-alt-text",
    "lint/a11y/useAnchorContent": "https://biomejs.dev/linter/rules/use-anchor-content",
    "lint/a11y/useAriaActivedescendantWithTabindex": "https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex",
    "lint/a11y/useAriaPropsForRole": "https://biomejs.dev/linter/rules/use-aria-props-for-role",
    "lint/a11y/useButtonType": "https://biomejs.dev/linter/rules/use-button-type",
    "lint/a11y/useFocusableInteractive": "https://biomejs.dev/linter/rules/use-focusable-interactive",
    "lint/a11y/useGenericFontNames": "https://biomejs.dev/linter/rules/use-generic-font-names",
    "lint/a11y/useHeadingContent": "https://biomejs.dev/linter/rules/use-heading-content",
    "lint/a11y/useHtmlLang": "https://biomejs.dev/linter/rules/use-html-lang",
    "lint/a11y/useIframeTitle": "https://biomejs.dev/linter/rules/use-iframe-title",
    "lint/a11y/useKeyWithClickEvents": "https://biomejs.dev/linter/rules/use-key-with-click-events",
    "lint/a11y/useKeyWithMouseEvents": "https://biomejs.dev/linter/rules/use-key-with-mouse-events",
    "lint/a11y/useMediaCaption": "https://biomejs.dev/linter/rules/use-media-caption",
    "lint/a11y/useSemanticElements": "https://biomejs.dev/linter/rules/use-semantic-elements",
    "lint/a11y/useValidAnchor": "https://biomejs.dev/linter/rules/use-valid-anchor",
    "lint/a11y/useValidAriaProps": "https://biomejs.dev/linter/rules/use-valid-aria-props",
    "lint/a11y/useValidAriaRole": "https://biomejs.dev/linter/rules/use-valid-aria-role",
    "lint/a11y/useValidAriaValues": "https://biomejs.dev/linter/rules/use-valid-aria-values",
    "lint/a11y/useValidLang": "https://biomejs.dev/linter/rules/use-valid-lang",
    "lint/complexity/noBannedTypes": "https://biomejs.dev/linter/rules/no-banned-types",
    "lint/complexity/noEmptyTypeParameters": "https://biomejs.dev/linter/rules/no-empty-type-parameters",
    "lint/complexity/noExcessiveCognitiveComplexity": "https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity",
    "lint/complexity/noExcessiveNestedTestSuites": "https://biomejs.dev/linter/rules/no-excessive-nested-test-suites",
    "lint/complexity/noExtraBooleanCast": "https://biomejs.dev/linter/rules/no-extra-boolean-cast",
    "lint/complexity/noForEach": "https://biomejs.dev/linter/rules/no-for-each",
    "lint/complexity/noMultipleSpacesInRegularExpressionLiterals": "https://biomejs.dev/linter/rules/no-multiple-spaces-in-regular-expression-literals",
    "lint/complexity/noStaticOnlyClass": "https://biomejs.dev/linter/rules/no-static-only-class",
    "lint/complexity/noThisInStatic": "https://biomejs.dev/linter/rules/no-this-in-static",
    "lint/complexity/noUselessCatch": "https://biomejs.dev/linter/rules/no-useless-catch",
    "lint/complexity/noUselessConstructor": "https://biomejs.dev/linter/rules/no-useless-constructor",
    "lint/complexity/noUselessEmptyExport": "https://biomejs.dev/linter/rules/no-useless-empty-export",
    "lint/complexity/noUselessFragments": "https://biomejs.dev/linter/rules/no-useless-fragments",
    "lint/complexity/noUselessLabel": "https://biomejs.dev/linter/rules/no-useless-label",
    "lint/complexity/noUselessLoneBlockStatements": "https://biomejs.dev/linter/rules/no-useless-lone-block-statements",
    "lint/complexity/noUselessRename": "https://biomejs.dev/linter/rules/no-useless-rename",
    "lint/complexity/noUselessStringConcat": "https://biomejs.dev/linter/rules/no-useless-string-concat",
    "lint/complexity/noUselessSwitchCase": "https://biomejs.dev/linter/rules/no-useless-switch-case",
    "lint/complexity/noUselessTernary": "https://biomejs.dev/linter/rules/no-useless-ternary",
    "lint/complexity/noUselessThisAlias": "https://biomejs.dev/linter/rules/no-useless-this-alias",
    "lint/complexity/noUselessTypeConstraint": "https://biomejs.dev/linter/rules/no-useless-type-constraint",
    "lint/complexity/noUselessUndefinedInitialization": "https://biomejs.dev/linter/rules/no-useless-undefined-initialization",
    "lint/complexity/noVoid": "https://biomejs.dev/linter/rules/no-void",
    "lint/complexity/noWith": "https://biomejs.dev/linter/rules/no-with",
    "lint/complexity/useArrowFunction": "https://biomejs.dev/linter/rules/use-arrow-function",
    "lint/complexity/useDateNow": "https://biomejs.dev/linter/rules/use-date-now",
    "lint/complexity/useFlatMap": "https://biomejs.dev/linter/rules/use-flat-map",
    "lint/complexity/useLiteralKeys": "https://biomejs.dev/linter/rules/use-literal-keys",
    "lint/complexity/useOptionalChain": "https://biomejs.dev/linter/rules/use-optional-chain",
    "lint/complexity/useRegexLiterals": "https://biomejs.dev/linter/rules/use-regex-literals",
    "lint/complexity/useSimpleNumberKeys": "https://biomejs.dev/linter/rules/use-simple-number-keys",
    "lint/complexity/useSimplifiedLogicExpression": "https://biomejs.dev/linter/rules/use-simplified-logic-expression",
    "lint/correctness/noChildrenProp": "https://biomejs.dev/linter/rules/no-children-prop",
    "lint/correctness/noConstAssign": "https://biomejs.dev/linter/rules/no-const-assign",
    "lint/correctness/noConstantCondition": "https://biomejs.dev/linter/rules/no-constant-condition",
    "lint/correctness/noConstantMathMinMaxClamp": "https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp",
    "lint/correctness/noConstructorReturn": "https://biomejs.dev/linter/rules/no-constructor-return",
    "lint/correctness/noEmptyCharacterClassInRegex": "https://biomejs.dev/linter/rules/no-empty-character-class-in-regex",
    "lint/correctness/noEmptyPattern": "https://biomejs.dev/linter/rules/no-empty-pattern",
    "lint/correctness/noFlatMapIdentity": "https://biomejs.dev/linter/rules/no-flat-map-identity",
    "lint/correctness/noGlobalObjectCalls": "https://biomejs.dev/linter/rules/no-global-object-calls",
    "lint/correctness/noInnerDeclarations": "https://biomejs.dev/linter/rules/no-inner-declarations",
    "lint/correctness/noInvalidBuiltinInstantiation": "https://biomejs.dev/linter/rules/no-invalid-builtin-instantiation",
    "lint/correctness/noInvalidConstructorSuper": "https://biomejs.dev/linter/rules/no-invalid-constructor-super",
    "lint/correctness/noInvalidDirectionInLinearGradient": "https://biomejs.dev/linter/rules/no-invalid-direction-in-linear-gradient",
    "lint/correctness/noInvalidGridAreas": "https://biomejs.dev/linter/rules/use-consistent-grid-areas",
    "lint/correctness/noInvalidNewBuiltin": "https://biomejs.dev/linter/rules/no-invalid-new-builtin",
    "lint/correctness/noInvalidPositionAtImportRule": "https://biomejs.dev/linter/rules/no-invalid-position-at-import-rule",
    "lint/correctness/noInvalidUseBeforeDeclaration": "https://biomejs.dev/linter/rules/no-invalid-use-before-declaration",
    "lint/correctness/noNewSymbol": "https://biomejs.dev/linter/rules/no-new-symbol",
    "lint/correctness/noNodejsModules": "https://biomejs.dev/linter/rules/no-nodejs-modules",
    "lint/correctness/noNonoctalDecimalEscape": "https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape",
    "lint/correctness/noPrecisionLoss": "https://biomejs.dev/linter/rules/no-precision-loss",
    "lint/correctness/noRenderReturnValue": "https://biomejs.dev/linter/rules/no-render-return-value",
    "lint/correctness/noSelfAssign": "https://biomejs.dev/linter/rules/no-self-assign",
    "lint/correctness/noSetterReturn": "https://biomejs.dev/linter/rules/no-setter-return",
    "lint/correctness/noStringCaseMismatch": "https://biomejs.dev/linter/rules/no-string-case-mismatch",
    "lint/correctness/noSwitchDeclarations": "https://biomejs.dev/linter/rules/no-switch-declarations",
    "lint/correctness/noUndeclaredDependencies": "https://biomejs.dev/linter/rules/no-undeclared-dependencies",
    "lint/correctness/noUndeclaredVariables": "https://biomejs.dev/linter/rules/no-undeclared-variables",
    "lint/correctness/noUnknownFunction": "https://biomejs.dev/linter/rules/no-unknown-function",
    "lint/correctness/noUnknownMediaFeatureName": "https://biomejs.dev/linter/rules/no-unknown-media-feature-name",
    "lint/correctness/noUnknownProperty": "https://biomejs.dev/linter/rules/no-unknown-property",
    "lint/correctness/noUnknownUnit": "https://biomejs.dev/linter/rules/no-unknown-unit",
    "lint/correctness/noUnmatchableAnbSelector": "https://biomejs.dev/linter/rules/no-unmatchable-anb-selector",
    "lint/correctness/noUnnecessaryContinue": "https://biomejs.dev/linter/rules/no-unnecessary-continue",
    "lint/correctness/noUnreachable": "https://biomejs.dev/linter/rules/no-unreachable",
    "lint/correctness/noUnreachableSuper": "https://biomejs.dev/docs/linter/rules/no-unreachable-super",
    "lint/correctness/noUnsafeFinally": "https://biomejs.dev/linter/rules/no-unsafe-finally",
    "lint/correctness/noUnsafeOptionalChaining": "https://biomejs.dev/linter/rules/no-unsafe-optional-chaining",
    "lint/correctness/noUnusedFunctionParameters": "https://biomejs.dev/linter/rules/no-unused-function-parameters",
    "lint/correctness/noUnusedImports": "https://biomejs.dev/linter/rules/no-unused-imports",
    "lint/correctness/noUnusedLabels": "https://biomejs.dev/linter/rules/no-unused-labels",
    "lint/correctness/noUnusedPrivateClassMembers": "https://biomejs.dev/linter/rules/no-unused-private-class-members",
    "lint/correctness/noUnusedVariables": "https://biomejs.dev/linter/rules/no-unused-variables",
    "lint/correctness/noVoidElementsWithChildren": "https://biomejs.dev/linter/rules/no-void-elements-with-children",
    "lint/correctness/noVoidTypeReturn": "https://biomejs.dev/linter/rules/no-void-type-return",
    "lint/correctness/useArrayLiterals": "https://biomejs.dev/linter/rules/use-array-literals",
    "lint/correctness/useExhaustiveDependencies": "https://biomejs.dev/linter/rules/use-exhaustive-dependencies",
    "lint/correctness/useHookAtTopLevel": "https://biomejs.dev/linter/rules/use-hook-at-top-level",
    "lint/correctness/useImportExtensions": "https://biomejs.dev/linter/rules/use-import-extensions",
    "lint/correctness/useIsNan": "https://biomejs.dev/linter/rules/use-is-nan",
    "lint/correctness/useJsxKeyInIterable": "https://biomejs.dev/linter/rules/use-jsx-key-in-iterable",
    "lint/correctness/useValidForDirection": "https://biomejs.dev/linter/rules/use-valid-for-direction",
    "lint/correctness/useYield": "https://biomejs.dev/linter/rules/use-yield",
    "lint/nursery/colorNoInvalidHex": "https://biomejs.dev/linter/rules/color-no-invalid-hex",
    "lint/nursery/noColorInvalidHex": "https://biomejs.dev/linter/rules/no-color-invalid-hex",
    "lint/nursery/noCommonJs": "https://biomejs.dev/linter/rules/no-common-js",
    "lint/nursery/noConsole": "https://biomejs.dev/linter/rules/no-console",
    "lint/nursery/noDescendingSpecificity": "https://biomejs.dev/linter/rules/no-descending-specificity",
    "lint/nursery/noDocumentCookie": "https://biomejs.dev/linter/rules/no-document-cookie",
    "lint/nursery/noDocumentImportInPage": "https://biomejs.dev/linter/rules/no-document-import-in-page",
    "lint/nursery/noDoneCallback": "https://biomejs.dev/linter/rules/no-done-callback",
    "lint/nursery/noDuplicateAtImportRules": "https://biomejs.dev/linter/rules/no-duplicate-at-import-rules",
    "lint/nursery/noDuplicateCustomProperties": "https://biomejs.dev/linter/rules/no-duplicate-custom-properties",
    "lint/nursery/noDuplicateElseIf": "https://biomejs.dev/linter/rules/no-duplicate-else-if",
    "lint/nursery/noDuplicateProperties": "https://biomejs.dev/linter/rules/no-duplicate-properties",
    "lint/nursery/noDuplicatedFields": "https://biomejs.dev/linter/rules/no-duplicated-fields",
    "lint/nursery/noDynamicNamespaceImportAccess": "https://biomejs.dev/linter/rules/no-dynamic-namespace-import-access",
    "lint/nursery/noEnum": "https://biomejs.dev/linter/rules/no-enum",
    "lint/nursery/noExportedImports": "https://biomejs.dev/linter/rules/no-exported-imports",
    "lint/nursery/noGlobalDirnameFilename": "https://biomejs.dev/linter/rules/no-global-dirname-filename",
    "lint/nursery/noHeadElement": "https://biomejs.dev/linter/rules/no-head-element",
    "lint/nursery/noHeadImportInDocument": "https://biomejs.dev/linter/rules/no-head-import-in-document",
    "lint/nursery/noImgElement": "https://biomejs.dev/linter/rules/no-img-element",
    "lint/nursery/noImportantInKeyframe": "https://biomejs.dev/linter/rules/no-important-in-keyframe",
    "lint/nursery/noInvalidDirectionInLinearGradient": "https://biomejs.dev/linter/rules/no-invalid-direction-in-linear-gradient",
    "lint/nursery/noInvalidGridAreas": "https://biomejs.dev/linter/rules/use-consistent-grid-areas",
    "lint/nursery/noInvalidPositionAtImportRule": "https://biomejs.dev/linter/rules/no-invalid-position-at-import-rule",
    "lint/nursery/noIrregularWhitespace": "https://biomejs.dev/linter/rules/no-irregular-whitespace",
    "lint/nursery/noMissingGenericFamilyKeyword": "https://biomejs.dev/linter/rules/no-missing-generic-family-keyword",
    "lint/nursery/noMissingVarFunction": "https://biomejs.dev/linter/rules/no-missing-var-function",
    "lint/nursery/noNestedTernary": "https://biomejs.dev/linter/rules/no-nested-ternary",
    "lint/nursery/noOctalEscape": "https://biomejs.dev/linter/rules/no-octal-escape",
    "lint/nursery/noPackagePrivateImports": "https://biomejs.dev/linter/rules/no-package-private-imports",
    "lint/nursery/noProcessEnv": "https://biomejs.dev/linter/rules/no-process-env",
    "lint/nursery/noProcessGlobal": "https://biomejs.dev/linter/rules/no-process-global",
    "lint/nursery/noReactSpecificProps": "https://biomejs.dev/linter/rules/no-react-specific-props",
    "lint/nursery/noRestrictedImports": "https://biomejs.dev/linter/rules/no-restricted-imports",
    "lint/nursery/noRestrictedTypes": "https://biomejs.dev/linter/rules/no-restricted-types",
    "lint/nursery/noSecrets": "https://biomejs.dev/linter/rules/no-secrets",
    "lint/nursery/noShorthandPropertyOverrides": "https://biomejs.dev/linter/rules/no-shorthand-property-overrides",
    "lint/nursery/noStaticElementInteractions": "https://biomejs.dev/linter/rules/no-static-element-interactions",
    "lint/nursery/noSubstr": "https://biomejs.dev/linter/rules/no-substr",
    "lint/nursery/noTemplateCurlyInString": "https://biomejs.dev/linter/rules/no-template-curly-in-string",
    "lint/nursery/noTsIgnore": "https://biomejs.dev/linter/rules/no-ts-ignore",
    "lint/nursery/noUndeclaredDependencies": "https://biomejs.dev/linter/rules/no-undeclared-dependencies",
    "lint/nursery/noUnknownAtRule": "https://biomejs.dev/linter/rules/no-unknown-at-rule",
    "lint/nursery/noUnknownFunction": "https://biomejs.dev/linter/rules/no-unknown-function",
    "lint/nursery/noUnknownMediaFeatureName": "https://biomejs.dev/linter/rules/no-unknown-media-feature-name",
    "lint/nursery/noUnknownProperty": "https://biomejs.dev/linter/rules/no-unknown-property",
    "lint/nursery/noUnknownPseudoClass": "https://biomejs.dev/linter/rules/no-unknown-pseudo-class-selector",
    "lint/nursery/noUnknownPseudoClassSelector": "https://biomejs.dev/linter/rules/no-unknown-pseudo-class-selector",
    "lint/nursery/noUnknownPseudoElement": "https://biomejs.dev/linter/rules/no-unknown-selector-pseudo-element",
    "lint/nursery/noUnknownSelectorPseudoElement": "https://biomejs.dev/linter/rules/no-unknown-selector-pseudo-element",
    "lint/nursery/noUnknownTypeSelector": "https://biomejs.dev/linter/rules/no-unknown-type-selector",
    "lint/nursery/noUnknownUnit": "https://biomejs.dev/linter/rules/no-unknown-unit",
    "lint/nursery/noUnmatchableAnbSelector": "https://biomejs.dev/linter/rules/no-unmatchable-anb-selector",
    "lint/nursery/noUnusedFunctionParameters": "https://biomejs.dev/linter/rules/no-unused-function-parameters",
    "lint/nursery/noUnwantedPolyfillio": "https://biomejs.dev/linter/rules/no-unwanted-polyfillio",
    "lint/nursery/noUselessEscapeInRegex": "https://biomejs.dev/linter/rules/no-useless-escape-in-regex",
    "lint/nursery/noUselessStringRaw": "https://biomejs.dev/linter/rules/no-useless-string-raw",
    "lint/nursery/noUselessUndefined": "https://biomejs.dev/linter/rules/no-useless-undefined",
    "lint/nursery/noValueAtRule": "https://biomejs.dev/linter/rules/no-value-at-rule",
    "lint/nursery/useAdjacentOverloadSignatures": "https://biomejs.dev/linter/rules/use-adjacent-overload-signatures",
    "lint/nursery/useAriaPropsSupportedByRole": "https://biomejs.dev/linter/rules/use-aria-props-supported-by-role",
    "lint/nursery/useAtIndex": "https://biomejs.dev/linter/rules/use-at-index",
    "lint/nursery/useBiomeSuppressionComment": "https://biomejs.dev/linter/rules/use-biome-suppression-comment",
    "lint/nursery/useCollapsedIf": "https://biomejs.dev/linter/rules/use-collapsed-if",
    "lint/nursery/useComponentExportOnlyModules": "https://biomejs.dev/linter/rules/use-components-only-module",
    "lint/nursery/useConsistentCurlyBraces": "https://biomejs.dev/linter/rules/use-consistent-curly-braces",
    "lint/nursery/useConsistentMemberAccessibility": "https://biomejs.dev/linter/rules/use-consistent-member-accessibility",
    "lint/nursery/useDeprecatedReason": "https://biomejs.dev/linter/rules/use-deprecated-reason",
    "lint/nursery/useExplicitFunctionReturnType": "https://biomejs.dev/linter/rules/use-explicit-function-return-type",
    "lint/nursery/useExplicitType": "https://biomejs.dev/linter/rules/use-explicit-function-return-type",
    "lint/nursery/useExportsLast": "https://biomejs.dev/linter/rules/use-exports-last",
    "lint/nursery/useGoogleFontDisplay": "https://biomejs.dev/linter/rules/use-google-font-display",
    "lint/nursery/useGoogleFontPreconnect": "https://biomejs.dev/linter/rules/use-google-font-preconnect",
    "lint/nursery/useGuardForIn": "https://biomejs.dev/linter/rules/use-guard-for-in",
    "lint/nursery/useImportRestrictions": "https://biomejs.dev/linter/rules/use-import-restrictions",
    "lint/nursery/useJsxCurlyBraceConvention": "https://biomejs.dev/linter/rules/use-jsx-curly-brace-convention",
    "lint/nursery/useNamedOperation": "https://biomejs.dev/linter/rules/use-named-operation",
    "lint/nursery/useNamingConvention": "https://biomejs.dev/linter/rules/use-naming-convention",
    "lint/nursery/useSortedClasses": "https://biomejs.dev/linter/rules/use-sorted-classes",
    "lint/nursery/useStrictMode": "https://biomejs.dev/linter/rules/use-strict-mode",
    "lint/nursery/useTrimStartEnd": "https://biomejs.dev/linter/rules/use-trim-start-end",
    "lint/nursery/useValidAutocomplete": "https://biomejs.dev/linter/rules/use-valid-autocomplete",
    "lint/performance/noAccumulatingSpread": "https://biomejs.dev/linter/rules/no-accumulating-spread",
    "lint/performance/noBarrelFile": "https://biomejs.dev/linter/rules/no-barrel-file",
    "lint/performance/noDelete": "https://biomejs.dev/linter/rules/no-delete",
    "lint/performance/noReExportAll": "https://biomejs.dev/linter/rules/no-re-export-all",
    "lint/performance/useTopLevelRegex": "https://biomejs.dev/linter/rules/use-top-level-regex",
    "lint/security/noDangerouslySetInnerHtml": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html-with-children",
    "lint/security/noGlobalEval": "https://biomejs.dev/linter/rules/no-global-eval",
    "lint/style/noArguments": "https://biomejs.dev/linter/rules/no-arguments",
    "lint/style/noCommaOperator": "https://biomejs.dev/linter/rules/no-comma-operator",
    "lint/style/noDefaultExport": "https://biomejs.dev/linter/rules/no-default-export",
    "lint/style/noDoneCallback": "https://biomejs.dev/linter/rules/no-done-callback",
    "lint/style/noImplicitBoolean": "https://biomejs.dev/linter/rules/no-implicit-boolean",
    "lint/style/noInferrableTypes": "https://biomejs.dev/linter/rules/no-inferrable-types",
    "lint/style/noNamespace": "https://biomejs.dev/linter/rules/no-namespace",
    "lint/style/noNamespaceImport": "https://biomejs.dev/linter/rules/no-namespace-import",
    "lint/style/noNegationElse": "https://biomejs.dev/linter/rules/no-negation-else",
    "lint/style/noNonNullAssertion": "https://biomejs.dev/linter/rules/no-non-null-assertion",
    "lint/style/noParameterAssign": "https://biomejs.dev/linter/rules/no-parameter-assign",
    "lint/style/noParameterProperties": "https://biomejs.dev/linter/rules/no-parameter-properties",
    "lint/style/noRestrictedGlobals": "https://biomejs.dev/linter/rules/no-restricted-globals",
    "lint/style/noShoutyConstants": "https://biomejs.dev/linter/rules/no-shouty-constants",
    "lint/style/noUnusedTemplateLiteral": "https://biomejs.dev/linter/rules/no-unused-template-literal",
    "lint/style/noUselessElse": "https://biomejs.dev/linter/rules/no-useless-else",
    "lint/style/noVar": "https://biomejs.dev/linter/rules/no-var",
    "lint/style/noYodaExpression": "https://biomejs.dev/linter/rules/no-yoda-expression",
    "lint/style/useAsConstAssertion": "https://biomejs.dev/linter/rules/use-as-const-assertion",
    "lint/style/useBlockStatements": "https://biomejs.dev/linter/rules/use-block-statements",
    "lint/style/useCollapsedElseIf": "https://biomejs.dev/linter/rules/use-collapsed-else-if",
    "lint/style/useConsistentArrayType": "https://biomejs.dev/linter/rules/use-consistent-array-type",
    "lint/style/useConsistentBuiltinInstantiation": "https://biomejs.dev/linter/rules/use-consistent-new-builtin",
    "lint/style/useConst": "https://biomejs.dev/linter/rules/use-const",
    "lint/style/useDefaultParameterLast": "https://biomejs.dev/linter/rules/use-default-parameter-last",
    "lint/style/useDefaultSwitchClause": "https://biomejs.dev/linter/rules/use-default-switch-clause",
    "lint/style/useEnumInitializers": "https://biomejs.dev/linter/rules/use-enum-initializers",
    "lint/style/useExplicitLengthCheck": "https://biomejs.dev/linter/rules/use-explicit-length-check",
    "lint/style/useExponentiationOperator": "https://biomejs.dev/linter/rules/use-exponentiation-operator",
    "lint/style/useExportType": "https://biomejs.dev/linter/rules/use-export-type",
    "lint/style/useFilenamingConvention": "https://biomejs.dev/linter/rules/use-filenaming-convention",
    "lint/style/useForOf": "https://biomejs.dev/linter/rules/use-for-of",
    "lint/style/useFragmentSyntax": "https://biomejs.dev/linter/rules/use-fragment-syntax",
    "lint/style/useImportType": "https://biomejs.dev/linter/rules/use-import-type",
    "lint/style/useLiteralEnumMembers": "https://biomejs.dev/linter/rules/use-literal-enum-members",
    "lint/style/useNamingConvention": "https://biomejs.dev/linter/rules/use-naming-convention",
    "lint/style/useNodeAssertStrict": "https://biomejs.dev/linter/rules/use-node-assert-strict",
    "lint/style/useNodejsImportProtocol": "https://biomejs.dev/linter/rules/use-nodejs-import-protocol",
    "lint/style/useNumberNamespace": "https://biomejs.dev/linter/rules/use-number-namespace",
    "lint/style/useNumericLiterals": "https://biomejs.dev/linter/rules/use-numeric-literals",
    "lint/style/useSelfClosingElements": "https://biomejs.dev/linter/rules/use-self-closing-elements",
    "lint/style/useShorthandArrayType": "https://biomejs.dev/linter/rules/use-shorthand-array-type",
    "lint/style/useShorthandAssign": "https://biomejs.dev/linter/rules/use-shorthand-assign",
    "lint/style/useShorthandFunctionType": "https://biomejs.dev/linter/rules/use-shorthand-function-type",
    "lint/style/useSingleCaseStatement": "https://biomejs.dev/linter/rules/use-single-case-statement",
    "lint/style/useSingleVarDeclarator": "https://biomejs.dev/linter/rules/use-single-var-declarator",
    "lint/style/useTemplate": "https://biomejs.dev/linter/rules/use-template",
    "lint/style/useThrowNewError": "https://biomejs.dev/linter/rules/use-throw-new-error",
    "lint/style/useThrowOnlyError": "https://biomejs.dev/linter/rules/use-throw-only-error",
    "lint/style/useWhile": "https://biomejs.dev/linter/rules/use-while",
    "lint/suspicious/noApproximativeNumericConstant": "https://biomejs.dev/linter/rules/no-approximative-numeric-constant",
    "lint/suspicious/noArrayIndexKey": "https://biomejs.dev/linter/rules/no-array-index-key",
    "lint/suspicious/noAssignInExpressions": "https://biomejs.dev/linter/rules/no-assign-in-expressions",
    "lint/suspicious/noAsyncPromiseExecutor": "https://biomejs.dev/linter/rules/no-async-promise-executor",
    "lint/suspicious/noCatchAssign": "https://biomejs.dev/linter/rules/no-catch-assign",
    "lint/suspicious/noClassAssign": "https://biomejs.dev/linter/rules/no-class-assign",
    "lint/suspicious/noCommentText": "https://biomejs.dev/linter/rules/no-comment-text",
    "lint/suspicious/noCompareNegZero": "https://biomejs.dev/linter/rules/no-compare-neg-zero",
    "lint/suspicious/noConfusingLabels": "https://biomejs.dev/linter/rules/no-confusing-labels",
    "lint/suspicious/noConfusingVoidType": "https://biomejs.dev/linter/rules/no-confusing-void-type",
    "lint/suspicious/noConsole": "https://biomejs.dev/linter/rules/no-console",
    "lint/suspicious/noConsoleLog": "https://biomejs.dev/linter/rules/no-console-log",
    "lint/suspicious/noConstEnum": "https://biomejs.dev/linter/rules/no-const-enum",
    "lint/suspicious/noControlCharactersInRegex": "https://biomejs.dev/linter/rules/no-control-characters-in-regex",
    "lint/suspicious/noDebugger": "https://biomejs.dev/linter/rules/no-debugger",
    "lint/suspicious/noDoubleEquals": "https://biomejs.dev/linter/rules/no-double-equals",
    "lint/suspicious/noDuplicateAtImportRules": "https://biomejs.dev/linter/rules/no-duplicate-at-import-rules",
    "lint/suspicious/noDuplicateCase": "https://biomejs.dev/linter/rules/no-duplicate-case",
    "lint/suspicious/noDuplicateClassMembers": "https://biomejs.dev/linter/rules/no-duplicate-class-members",
    "lint/suspicious/noDuplicateFontNames": "https://biomejs.dev/linter/rules/no-font-family-duplicate-names",
    "lint/suspicious/noDuplicateJsxProps": "https://biomejs.dev/linter/rules/no-duplicate-jsx-props",
    "lint/suspicious/noDuplicateObjectKeys": "https://biomejs.dev/linter/rules/no-duplicate-object-keys",
    "lint/suspicious/noDuplicateParameters": "https://biomejs.dev/linter/rules/no-duplicate-parameters",
    "lint/suspicious/noDuplicateSelectorsKeyframeBlock": "https://biomejs.dev/linter/rules/no-duplicate-selectors-keyframe-block",
    "lint/suspicious/noDuplicateTestHooks": "https://biomejs.dev/linter/rules/no-duplicate-test-hooks",
    "lint/suspicious/noEmptyBlock": "https://biomejs.dev/linter/rules/no-empty-block",
    "lint/suspicious/noEmptyBlockStatements": "https://biomejs.dev/linter/rules/no-empty-block-statements",
    "lint/suspicious/noEmptyInterface": "https://biomejs.dev/linter/rules/no-empty-interface",
    "lint/suspicious/noEvolvingTypes": "https://biomejs.dev/linter/rules/no-evolving-types",
    "lint/suspicious/noExplicitAny": "https://biomejs.dev/linter/rules/no-explicit-any",
    "lint/suspicious/noExportsInTest": "https://biomejs.dev/linter/rules/no-exports-in-test",
    "lint/suspicious/noExtraNonNullAssertion": "https://biomejs.dev/linter/rules/no-extra-non-null-assertion",
    "lint/suspicious/noFallthroughSwitchClause": "https://biomejs.dev/linter/rules/no-fallthrough-switch-clause",
    "lint/suspicious/noFocusedTests": "https://biomejs.dev/linter/rules/no-focused-tests",
    "lint/suspicious/noFunctionAssign": "https://biomejs.dev/linter/rules/no-function-assign",
    "lint/suspicious/noGlobalAssign": "https://biomejs.dev/linter/rules/no-global-assign",
    "lint/suspicious/noGlobalIsFinite": "https://biomejs.dev/linter/rules/no-global-is-finite",
    "lint/suspicious/noGlobalIsNan": "https://biomejs.dev/linter/rules/no-global-is-nan",
    "lint/suspicious/noImplicitAnyLet": "https://biomejs.dev/linter/rules/no-implicit-any-let",
    "lint/suspicious/noImportAssign": "https://biomejs.dev/linter/rules/no-import-assign",
    "lint/suspicious/noImportantInKeyframe": "https://biomejs.dev/linter/rules/no-important-in-keyframe",
    "lint/suspicious/noLabelVar": "https://biomejs.dev/linter/rules/no-label-var",
    "lint/suspicious/noMisleadingCharacterClass": "https://biomejs.dev/linter/rules/no-misleading-character-class",
    "lint/suspicious/noMisleadingInstantiator": "https://biomejs.dev/linter/rules/no-misleading-instantiator",
    "lint/suspicious/noMisplacedAssertion": "https://biomejs.dev/linter/rules/no-misplaced-assertion",
    "lint/suspicious/noMisrefactoredShorthandAssign": "https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign",
    "lint/suspicious/noPrototypeBuiltins": "https://biomejs.dev/linter/rules/no-prototype-builtins",
    "lint/suspicious/noReactSpecificProps": "https://biomejs.dev/linter/rules/no-react-specific-props",
    "lint/suspicious/noRedeclare": "https://biomejs.dev/linter/rules/no-redeclare",
    "lint/suspicious/noRedundantUseStrict": "https://biomejs.dev/linter/rules/no-redundant-use-strict",
    "lint/suspicious/noSelfCompare": "https://biomejs.dev/linter/rules/no-self-compare",
    "lint/suspicious/noShadowRestrictedNames": "https://biomejs.dev/linter/rules/no-shadow-restricted-names",
    "lint/suspicious/noShorthandPropertyOverrides": "https://biomejs.dev/linter/rules/no-shorthand-property-overrides",
    "lint/suspicious/noSkippedTests": "https://biomejs.dev/linter/rules/no-skipped-tests",
    "lint/suspicious/noSparseArray": "https://biomejs.dev/linter/rules/no-sparse-array",
    "lint/suspicious/noSuspiciousSemicolonInJsx": "https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx",
    "lint/suspicious/noThenProperty": "https://biomejs.dev/linter/rules/no-then-property",
    "lint/suspicious/noUnsafeDeclarationMerging": "https://biomejs.dev/linter/rules/no-unsafe-declaration-merging",
    "lint/suspicious/noUnsafeNegation": "https://biomejs.dev/linter/rules/no-unsafe-negation",
    "lint/suspicious/useAwait": "https://biomejs.dev/linter/rules/use-await",
    "lint/suspicious/useDefaultSwitchClauseLast": "https://biomejs.dev/linter/rules/use-default-switch-clause-last",
    "lint/suspicious/useErrorMessage": "https://biomejs.dev/linter/rules/use-error-message",
    "lint/suspicious/useGetterReturn": "https://biomejs.dev/linter/rules/use-getter-return",
    "lint/suspicious/useIsArray": "https://biomejs.dev/linter/rules/use-is-array",
    "lint/suspicious/useNamespaceKeyword": "https://biomejs.dev/linter/rules/use-namespace-keyword",
    "lint/suspicious/useNumberToFixedDigitsArgument": "https://biomejs.dev/linter/rules/use-number-to-fixed-digits-argument",
    "lint/suspicious/useValidTypeof": "https://biomejs.dev/linter/rules/use-valid-typeof",
    // end lint rules
    // start assist actions
    "assist/source/useSortedKeys": "https://biomejs.dev/linter/actions/use-sorted-keys",
    // end assist actions
    ; // start syntax rules
    "syntax/correctness/noTypeOnlyImportAttributes",
    "syntax/correctness/noSuperWithoutExtends",
    "syntax/correctness/noInitializerWithDefinite",
    "syntax/correctness/noDuplicatePrivateClassMembers",
    // end syntax rules

    // General categories
    "files/missingHandler",
    "format",
    "check",
    "ci",
    "stdin",
    "configuration",
    "organizeImports",
    "assist",
    "migrate",
    "deserialize",
    "plugin",
    "project",
    "search",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",
    "reporter/parse",
    "reporter/format",
    "reporter/analyzer",
    "reporter/organizeImports",
    // parse categories
    "parse",

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
    "suppressions/incorrect",

    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
