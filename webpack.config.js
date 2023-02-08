const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { WebpackConfigDumpPlugin } = require("webpack-config-dump-plugin");
const webpack = require('webpack');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    main: "./js/bootstrap.js"
  },
  output: {
    path: dist,
    filename: "worker.js",
    globalObject: 'self',
  },
  devServer: {
    contentBase: dist,
  },
  optimization: {
    minimize: false,
    splitChunks: {

    },
  },
  performance: {
    hints: false,
    maxEntrypointSize: 393216,
    maxAssetSize: 0
  },
  module: {
    rules: [
        {
            test: /\.wasm$/,
            type: "asset/inline",
        },
        
    ],
  },
  plugins: [

    new webpack.optimize.LimitChunkCountPlugin({
      maxChunks: 1,
    })

  ]
  
};
