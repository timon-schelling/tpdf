use timpl::*;

fn main() {
    println!("{}", template());
}

pub(crate) fn template() -> String {

    let domain = "dev.app.example.com";

    let name = {
        let mut res = domain.split('.').collect::<Vec<&str>>();
        res.reverse();
        res.join("-")
    };

    let frontend_needs_redirect = true;
    let frontend_service_namespace = "default";
    let frontend_service_name = "frontend";
    let frontend_service_port = 80;

    let monitoring_path = "/monitoring";
    let monitoring_service_namespace = "monitoring";
    let monitoring_service_name = "monitoring";
    let monitoring_service_port = 80;

    timpl! {

        apiVersion: k8s.nginx.org/v1
        kind: VirtualServer
        metadata:
          name: { name }
          namespace: ingress
        spec:
          host: { domain }
          tls:
            secret: { name }-tls
          routes:
          - path: /
            route: { frontend_service_namespace }/{ name }-{ frontend_service_name }
          - path: { monitoring_path }
            route: { monitoring_service_namespace }/{ name }-{ monitoring_service_name }

        ---
        apiVersion: k8s.nginx.org/v1
        kind: VirtualServerRoute
        metadata:
          name: { name }-{ frontend_service_name }
          namespace: { frontend_service_namespace }
        spec:
          host: { domain }
          upstreams:
          - name: { frontend_service_name }
            service: { frontend_service_name }
            port: { frontend_service_port }
          subroutes:
          {
              timpl_if!(frontend_needs_redirect, {
                  - path: =/
                    action:
                      redirect:
                        url: { "${scheme}://${host}/index.html" }
              })
          }
          - path: /
            action:
              pass: { frontend_service_name }

        ---
        apiVersion: k8s.nginx.org/v1
        kind: VirtualServerRoute
        metadata:
          name: { name }-{ monitoring_service_name }
          namespace: { monitoring_service_namespace }
        spec:
          host: { domain }
          upstreams:
          - name: { monitoring_service_name }
            service: { monitoring_service_name }
            port: { monitoring_service_port }
          subroutes:
          - path: { monitoring_path }
            action:
              pass: { monitoring_service_name }

    }
}
