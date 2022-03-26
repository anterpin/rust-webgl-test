const path = require('path');

// console.log(path.resolve(__dirname,'dist'));
module.exports = {
    entry: "./index.js",
    output:{
        path: path.resolve(__dirname,"dist"),
        filename: "index.js",
    },
    experiments: {
        asyncWebAssembly: true,
    },
    devServer:{
        static:{
            directory: path.join(__dirname)
        }
    },
    mode: "development"
};