module.exports = {
  env: {
    es6: true,
    browser: true,
  },
  parser: 'babel-eslint',
  extends: ['airbnb-base', 'plugin:prettier/recommended'],
  rules: {
    'linebreak-style': ['error', 'unix'],
    'arrow-parens': ['error', 'as-needed'],
    'no-param-reassign': ['error', { props: false }],
    'func-style': ['error', 'declaration', { allowArrowFunctions: true }],
    'no-use-before-define': ['error', { functions: false }],
    'no-shadow': ['error', { builtinGlobals: true }],
  },
};
