const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');

module.exports = {
  configureWebpack: config => {
    // See: https://stackoverflow.com/questions/55258355/vue-clis-type-checking-service-ignores-memory-limits#answer-55810460

    const tsPlugin = config.plugins.filter(
      p => p instanceof ForkTsCheckerWebpackPlugin,
    )[0];

    config.plugins = config.plugins.filter(
      p => !(p instanceof ForkTsCheckerWebpackPlugin),
    );

    const tsOptions = { ...tsPlugin.options, memoryLimit: 1024 };

    config.plugins.push(new ForkTsCheckerWebpackPlugin(tsOptions));
  },

  devServer: {
    proxy: {
      '/api': {
        target: 'http://localhost:7100',
      },
    },
  },
};
