require('@babel/register');
require('dotenv').config();

const path = require('path');
const webpack = require('webpack');
const SimpleProgressWebpackPlugin = require('simple-progress-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');

const outputPath = path.resolve(__dirname, './dist');

module.exports = {
  entry: {
    index: ['./src/index.js'],
  },
  target: 'web', // tells webpack that this build will be run in browsers
  output: {
    filename: '[name].js',
    libraryTarget: 'umd',
    path: outputPath,
  },
  module: {
    strictExportPresence: true,
    rules: [
      { 
        oneOf: [{
          test: /\.jsx?$/, // regex that matches the files that this loader should be handling
          exclude: /node_modules/,
          loaders: 'babel-loader',
          options: {
            babelrc: false,
            cacheDirectory: true,
            cacheCompression: true,
            presets: [
              ['@babel/preset-env', { modules: false, useBuiltIns: 'entry' }],
            ],
          },
        }],
      },
      // this rule is required so that webpack use file-loader for wasm file, 
      // and not use its own rules
      // (https://github.com/webpack/webpack/issues/6725#issuecomment-391237775)
      {
        test: /\.(wasm)$/,
        type: 'javascript/auto',
        use: [
          {
            loader: 'file-loader',
            options: {
              name (_file) {
                if (process.env.NODE_ENV === 'development') {
                  return '[path][name].[ext]';
                } else {
                  return '[contenthash].[ext]';
                }
              }
            },
          }
        ]
      },
    ],
  },

  plugins: [
    new SimpleProgressWebpackPlugin(),
    new webpack.HashedModuleIdsPlugin(),
    new CopyPlugin([
      { from: './src/plugins/RqrrWasmPlugin.js', to: path.join(outputPath, 'plugins/index.js') },
    ]),
  ],
  optimization: {
    nodeEnv: 'production',
    minimize: true,
  },
  mode: 'production',
};
