type Query = { [key: string]: string }

type RouteContext = {
    ctx: {
        params: Array<string>,
        path: string,
        query: Query
    }
    [key: string]: unknown
};
type RouteHandler = (ctx: RouteContext) => void
type Route = {
    refresh: boolean, 
    handler: RouteHandler
};

type RouterMap = {
    [key: string]: Route;
};

class Router {
    routes: RouterMap = {}
    defaultRoute: Route = {
        refresh: true,
        handler: () => {}
    }

    constructor() {
        window.__CREAM__.router = {
            routes: {},
            defaultRoute: this.defaultRoute
        }
    }

    setHandle(path: string, handler: RouteHandler, refresh: boolean) {
        this.routes[path] = { handler, refresh };
    }

    setDefaultHandler(handler: RouteHandler, refresh: boolean) {
        this.defaultRoute = { refresh, handler }
    }

    serve(location: Location = window.location) {
        const pathname = location.pathname
        const query = new URLSearchParams(location.search);
        const route = this.routes[pathname] ?? this.defaultRoute;

        if (route.refresh) 
            document.body.replaceChildren()

        route.handler({
            ctx: {params: [], path: pathname, query: Object.fromEntries(query)}
        })

        window.addEventListener("popstate", () => this.serve())
    }

    static visit(path: string) {
        history.pushState({}, "", path);
        if (window.__CREAM__.router) {
            window.__CREAM__.router.serve(window.location)
        }
    }
}

export { Route, RouteContext, RouteHandler, Router, RouterMap }
