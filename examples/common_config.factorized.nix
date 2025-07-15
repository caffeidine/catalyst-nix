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
{
  test_get = test rec {
    response = httpRequest {
      method = "GET";
      url = config.baseUrl + "/get";
      headers = config.headers;
    };
    assertions = with response; [
      (status == 200)
      (json ? args && json.args == { })
      (json ? headers && json.headers."Host" == "httpbin.org")
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
      };
      assertions = with response; [
        (status == 200)
        (json ? token && json.token == testToken)
      ];
    };
}
