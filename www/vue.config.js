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
          count_all_time: 6,
          count_6_days: 6,
          last_email: "2024-11-12 11:12:32.313971508",
        },
        {
          email: "second@jdc.1-ml.com",
          status: "ForwardAsText",
          count_all_time: 0,
          count_6_days: 0,
          last_email: null,
        },
        {
          email: "third@jdc.1-ml.com",
          status: "Block",
          count_all_time: 6,
          count_6_days: 0,
          last_email: "2024-11-13 11:12:32.313971508",
        },
        {
          email: "fourth@jdc.1-ml.com",
          status: "Block",
          count_all_time: 14,
          count_6_days: 10,
          last_email: "2024-11-12 11:12:32.313971508",
        },
        {
          email: "a.very.long.address.to.see.what.happen@jdc.1-ml.com",
          status: "Block",
          count_all_time: 2,
          count_6_days: 0,
          last_email: null,
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
