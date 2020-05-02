const webpack = require('webpack')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const path = require('path')

module.exports = (env, args) => {
  const isProductionMode = (args.mode === 'production')

  return {
    entry: './html/index.js',
    module: {
      rules: [
        {
          test: /\.(png|jpe?g|gif)$/i,
          use: [
            {
              loader: 'file-loader'
            }
          ]
        }
      ]
    },
    output: {
      path: path.resolve(__dirname, 'dist'),
      publicPath: path.resolve(__dirname, '/'),
      filename: isProductionMode ? '[name].[contenthash].js' : '[name].[hash].js'
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: './html/index.html'
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '.')
      }),
      new CopyWebpackPlugin([
        { from: './html/*.png', flatten: true }
      ]),
      new webpack.ProvidePlugin({
        TextDecoder: ['text-encoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder']
      })
    ]
  }
}
