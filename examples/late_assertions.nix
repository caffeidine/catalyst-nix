rec {
  test_get = builtins.test {
    response = builtins.httpRequest {
      method = "GET";
      url = "https://httpbin.org/get";
      headers = {
        "User-Agent" = "Catalyst Test";
        "Content-Type" = "application/json";
      };
    };
    assertions = [ ];
  };

  test_auth = builtins.test rec {
    response = builtins.httpRequest {
      method = "GET";
      url = "https://httpbin.org/bearer";
      headers = {
        "User-Agent" = "Catalyst Test";
        "Content-Type" = "application/json";
        "Authorization" = "Bearer Catalyst Testing Token";
      };
    };
    assertions = [
      # late assertions for `test_get`
      (test_get.response.status == 200)
      (test_get.response.json ? args && test_get.response.json.args == { })
      (test_get.response.json ? headers && test_get.response.json.headers."Host" == "httpbin.org")

      (response.status == 200)
      (response.json ? token && response.json.token == "Catalyst Testing Token")
    ];
  };
}
