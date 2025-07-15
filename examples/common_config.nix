let
  config = {
    baseUrl = "https://httpbin.org";
    headers = {
      "User-Agent" = "Catalyst Test";
      "Content-Type" = "application/json";
    };
  };
in
{
  test_get = builtins.test rec {
    response = builtins.httpRequest {
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

  test_auth = builtins.test rec {
    response = builtins.httpRequest {
      method = "GET";
      url = config.baseUrl + "/bearer";
      headers = config.headers // {
        "Authorization" = "Bearer Catalyst Testing Token";
      };
      timeout = 2000;
    };
    assertions = [
      (response.status == 200)
      (response.json ? token && response.json.token == "Catalyst Testing Token")
    ];
  };
}
