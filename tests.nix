with builtins;
rec {
  config = {
    baseUrl = "https://httpbin.org";
    headers = {
      "User-Agent" = "Catalyst Test";
      "Content-Type" = "application/json";
    };
  };

  test_get = test rec {
    response = httpRequest {
      method = "GET";
      url = config.baseUrl + "/get";
      headers = config.headers;
      timeout = 2000;
    };
    assertions = [
      (response.status == 200)
      (response.json ? args && response.json.args == { })
      (response.json ? headers && response.json.headers."Host" == "httpbin.org")
    ];
  };

  test_auth =
    let
      testToken = "Catalyst Testing Token";
    in
    test rec {
      response = httpRequest {
        method = "GET";
        url = config.baseUrl + "/bearer";
        headers = config.headers // {
          "Authorization" = "Bearer " + testToken;
        };
        timeout = 2000;
      };
      assertions = [
        (response.status == 200)
        (response.json ? token && response.json.token == testToken)
      ];
    };
}
