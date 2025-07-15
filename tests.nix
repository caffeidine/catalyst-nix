with builtins;
rec {
  config = {
    baseUrl = "https://httpbin.org";
    headers = {
      "User-Agent" = "Catalyst Test";
      "Content-Type" = "application/json";
    };
  };

  test_get = rec {
    response = httpRequest {
      method = "GET";
      url = config.baseUrl + "/get";
      headers = config.headers;
      timeout = 2000;
    };
    assertions = (
      assert response.status == 200;
      assert response.json != null;
      assert response.json.args == { };
      assert response.json.headers."Host" == "httpbin.org";
      null
    );
  };

  test_auth =
    let
      token = "Catalyst Testing Token";
    in
    rec {
      response = httpRequest {
        method = "GET";
        url = config.baseUrl + "/bearer";
        headers = config.headers // {
          "Authorization" = "Bearer " + token;
        };
        timeout = 2000;
      };
      assertions = (
        assert response.status == 200;
        assert response.json != null;
        assert response.json.token == token;
        null
      );
    };
}
