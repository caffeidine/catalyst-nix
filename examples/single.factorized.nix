with builtins;
test rec {
  response = httpRequest {
    method = "GET";
    url = "https://httpbin.org/get";
    headers = {
      "User-Agent" = "Catalyst Test";
      "Content-Type" = "application/json";
    };
  };
  assertions = with response; [
    (status == 200)
    (json ? args && json.args == { })
    (json ? headers && json.headers."Host" == "httpbin.org")
  ];
}
