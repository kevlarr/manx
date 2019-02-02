const path = require('path');

module.exports = {
  publicPath: '/app/',

  devServer: {
    proxy: 'http://localhost:7100',
  },
};
