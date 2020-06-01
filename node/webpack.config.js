const HtmlWebpackPlugin = require('html-webpack-plugin')
const path = require('path')

const MODE = "development";
const enableSourceMap = MODE === "development";

module.exports = {
    mode: MODE,
    entry: path.resolve(__dirname, "src/index.js"),
    resolve: {
        extensions: ['.js', '.wasm', '.css'],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./src/index.html"
        })
    ],
    output: {
        path: path.resolve(__dirname, "output"),
        filename: "bundle.js"
    },
    devServer: {
        inline: true,
        host: "0.0.0.0"
    },
    module: {
        rules: [
            {
                test: /\.css/,
                use: [
                    "style-loader",
                    {
                        loader: "css-loader",
                        options: {
                            sourceMap: enableSourceMap,
                            url: false
                        }
                    }
                ]
            },
            {
                test: /\.scss/,
                use: [
                    "style-loader",
                    {
                        loader: "css-loader",
                        options: {
                            sourceMap: enableSourceMap,
                            url: false,
                            importLoaders: 2
                        }
                    },
                    {
                        loader: "sass-loader",
                        options: {
                            sourceMap: enableSourceMap
                        }
                    }
                ]
            }
        ]
    }
}