const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');

module.exports = {
    entry: `./src/entry.ts`,
    output: {
        path: path.join(__dirname, `static`)
    },
    devServer: {
        contentBase: path.join(__dirname, "static"),
        hot: true,
        historyApiFallback: true
    },
    module: {
        rules: [
            {
                test: /\.scss/,
                use: [
                    MiniCssExtractPlugin.loader,
                    "css-loader",
                    'postcss-loader',
                    "sass-loader",
                ]
            },
            {
                test: /\.(m?js|ts)$/,
                exclude: /(node_modules|bower_components)/,
                use:  "swc-loader"
            }
        ]
    },
    plugins: [new MiniCssExtractPlugin({
        filename: `styles.css`,
    })],
    optimization: {
        minimizer: [
          new CssMinimizerPlugin(),
          new HtmlWebpackPlugin({
              minify: true,
              template: path.join(__dirname, `src`, `template.html`)
          })
        ],
    },
}