const path = require('path');

module.exports = {
  entry: {
    bundle: './app/index.js',
  },

  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'public'),
  },

  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.scss$/,
        use: [
          {
            loader: 'style-loader',
          },
          {
            loader: 'css-loader',
            options: { minimize: true },
          },
          {
            loader: 'sass-loader',
          },
        ],
      },
      {
        test: /\.js$/,
        exclude: [/node_modules/],
        use: [{ loader: 'babel-loader' }],
      },
    ],
  },
};
