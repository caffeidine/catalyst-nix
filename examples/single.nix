builtins.test rec {
  response = builtins.httpRequest {
    method = "GET";
    url = "https://httpbin.org/get";
    headers = {
      "User-Agent" = "Catalyst Test";
      "Content-Type" = "application/json";
    };
  };
  assertions = [
    (response.status == 200)
    (response.json ? args && response.json.args == { })
    (response.json ? headers && response.json.headers."Host" == "httpbin.org")
  ];
}
