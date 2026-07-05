type Query = { [key: string]: string };

type RouteContext = {
    ctx: {
        params: string[];
        path: string;
        query: Query;
    };
    [key: string]: unknown;
};

type RouteHandler = (ctx: RouteContext) => void;

type Route = {
    refresh: boolean;
    handler: RouteHandler;
};

type RouterMap = {
    [key: string]: Route;
};

function preprocessPath(path: string): string {
    return "/" + path
        .split("/")
        .filter(Boolean)
        .join("/");
}

declare global {
    interface Window {
        __CREAM__?: {
            router?: {
                routes: RouterMap;
                defaultRoute: Route;
                serve: (location?: Location) => void;
            };
        };
    }
}

class Router {
    defaultRoute: Route = {
        refresh: true,
        handler: () => {}
    };

    constructor() {
        window.__CREAM__ ??= {};

        window.__CREAM__.router = {
            routes: {},
            defaultRoute: this.defaultRoute,
            serve: this.serve.bind(this)
        };

        window.addEventListener("popstate", () => {
            this.serve();
        });
    }

    setHandle(
        path: string,
        handler: RouteHandler,
        refresh: boolean
    ) {
        window.__CREAM__!.router!.routes[
            preprocessPath(path)
        ] = {
            handler,
            refresh
        };
    }

    setDefaultHandler(
        handler: RouteHandler,
        refresh: boolean
    ) {
        window.__CREAM__!.router!.defaultRoute = {
            handler,
            refresh
        };
    }

    serve(location: Location = window.location) {
        const pathname = preprocessPath(location.pathname);

        const query = Object.fromEntries(
            new URLSearchParams(location.search)
        );

        const router = window.__CREAM__?.router;

        if (!router) {
            return;
        }

        const route =
            router.routes[pathname] ??
            router.defaultRoute;

        if (route.refresh) {
            document.body.replaceChildren();
        }

        route.handler({
            ctx: {
                params: [],
                path: pathname,
                query
            }
        });
    }

    static visit(path: string) {
        const router = window.__CREAM__?.router;

        if (router) {
            history.pushState({}, "", path);
            router.serve();
        } else {
            window.location.href = path;
        }
    }
}

export {
    Route,
    RouteContext,
    RouteHandler,
    Router,
    RouterMap
};