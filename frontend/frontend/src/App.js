import React, {Component} from "react";
import {
    Route,
    NavLink,
    HashRouter
} from "react-router-dom";
import Home from "./Home";
import Blog from "./Blog";
import Contact from "./Contact";

class App extends Component {
    render() {
        return (
            <HashRouter>
                <div>
                    <h1>Blog</h1>
                    <ul className="header">
                        <li><NavLink exact to="/">Home</NavLink></li>
                        {/*<li><NavLink to="/blog">Blog</NavLink></li>*/}
                        <li><NavLink to="/contact">Kontakt</NavLink></li>
                    </ul>
                    <div className="content">
                        <Route exact path="/" component={Blog}/>
                        {/*<Route path="/blog" component={Blog}/>*/}
                        <Route path="/contact" component={Contact}/>
                    </div>
                </div>
            </HashRouter>
        );
    }
}

export default App;
