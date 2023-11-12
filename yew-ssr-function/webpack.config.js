const path = require("path");
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
  optimization: {
    minimize: true,
  },
  performance: {
    maxEntrypointSize: 2097152,
    maxAssetSize: 2097152
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



