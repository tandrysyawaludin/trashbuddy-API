"use strict";

module.exports = {
  module: {
    loaders: [
      {
        test: /\.scss$/,
        loader: "style-loader!css-loader!sass-loader"
      },
      // ...
    ]
  }
};