/**
 * @type {import('eslint-define-config').ESLintConfig}
 */
module.exports = {
  env: { node: true, browser: false },
  parser: '@typescript-eslint/parser',
  plugins: ['import', '@typescript-eslint', 'prettier'],
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:import/recommended',
    'plugin:prettier/recommended',
  ],
  parserOptions: {
    sourceType: 'module',
    project: ['./tsconfig.json', './tsconfig.node.json'],
  },
  rules: {
    'import/order': 'error',
    'import/no-unresolved': 'off',
    'prefer-template': 'error',
    '@typescript-eslint/no-unused-vars': 'error',
    '@typescript-eslint/no-non-null-assertion': 'error',
    'no-console': ['error', { allow: ['error'] }],
    'lines-between-class-members': ['error', 'always', { exceptAfterSingleLine: true }],
    'prettier/prettier': 'error',
    '@typescript-eslint/no-floating-promises': ['error'],
    '@typescript-eslint/no-explicit-any': ['error'],
    '@typescript-eslint/explicit-module-boundary-types': 'off',
    '@typescript-eslint/prefer-optional-chain': 'error',
    '@typescript-eslint/consistent-type-imports': 'error',
  },
};
