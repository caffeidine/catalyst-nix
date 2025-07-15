with builtins;
rec {
  config = {
    baseUrl = "https://jsonplaceholder.typicode.com";
    headers = {
      "User-Agent" = "Catalyst Test";
      "Content-Type" = "application/json";
    };
  };

  get_post = rec {
    response = httpRequest {
      method = "GET";
      url = config.baseUrl + "/posts/1";
      headers = config.headers;
      timeout = 2000;
    };
    assertions = (
      assert response.status == 200;
      assert response.json != null;
      null
    );
  };

  create_post = rec {
    response = httpRequest {
      method = "POST";
      url = config.baseUrl + "/posts";
      headers = config.headers // {
        "Authorization" = "Bearer " + toString get_post.response.json.id;
      };
      timeout = 2000;
    };
    assertions = (
      assert response.status == 201;
      assert response.json != null;
      null
    );
  };
}
