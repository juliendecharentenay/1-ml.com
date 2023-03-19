const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  pages: {
    index: {
      entry: './src/pages/index/main.js',
      filename: 'index.html',
      title: '1-ml | Email Privacy',
    },
    home: {
      entry: './src/pages/home/main.js',
      filename: 'home.html',
      title: '1-ml | Home',
    },
  },
  devServer: {
    proxy: {},
    setupMiddlewares: (middlewares, devServer) => {
      let api_me = {
          user_id: "34ec0a66-1cc7-442b-a3c7-f7583e64dbb8",
          prefix: "pre",
          email: "julien.decharentenay@gmail.com",
          status: "Active",
          date_created: "2022-06-05T18:22:14.272585710Z"
      };

      devServer.app.get("/api/me", (_, response) => {
        console.log("get /api/me request");
        response.send(JSON.stringify(api_me));
      });

      devServer.app.patch("/api/me", (request, response) => {
        console.log("patch /api/me request");
        let body = '';
        request.on('data', (c) => {body += c;});
        request.on('end', () => {
          api_me = {...api_me, ...JSON.parse(body)};
          response.send(JSON.stringify(api_me));
        });
      });

      let api_emails = [
        {
          email: "first@jdc.1-ml.com",
          status: "Forward",
        },
        {
          email: "second@jdc.1-ml.com",
          status: "ForwardAsText",
        },
        {
          email: "third@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
        },
        {
          email: "a.very.long.address.to.see.what.happen@jdc.1-ml.com",
          status: "Block",
        },
      ];
      devServer.app.get("/api/email", (request, response) => {
        console.log("get /api/email");
        response.send(JSON.stringify(api_emails));
      });
      devServer.app.patch(`/api/email/${encodeURIComponent("first@jdc.1-ml.com")}`, (request, response) => {
        console.log("patch /api/email/first....");
        let body = '';
        request.on('data', (c) => {body += c;});
        request.on('end', () => {
          let h = JSON.parse(body);
          api_emails[0].status = h.status;
          response.send(JSON.stringify(api_emails[0]));
        });
      });

      return middlewares;
    }
  },
  transpileDependencies: true,
  productionSourceMap: true, // false,
})
