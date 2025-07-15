let
  config = {
    baseUrl = "https://httpbin.org";
    headers = {
      "User-Agent" = "Catalyst Test";
      "Content-Type" = "application/json";
    };
  };
in
with builtins;
rec {
  test_get = test {
    response = httpRequest {
      method = "GET";
      url = config.baseUrl + "/get";
      headers = config.headers;
    };
    assertions = [ ];
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
      };
      assertions = with response; [
        # late assertions for `test_get`
        (test_get.response.status == 200)
        (test_get.response.json ? args && test_get.response.json.args == { })
        (test_get.response.json ? headers && test_get.response.json.headers."Host" == "httpbin.org")

        (status == 200)
        (json ? token && json.token == "Catalyst Testing Token")
      ];
    };
}
