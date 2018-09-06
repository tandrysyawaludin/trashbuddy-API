import React, { Component } from 'react';
import { BrowserRouter as Router, Route, Switch, Redirect } from 'react-router-dom';

import SignIn from '../pages/SignIn';
import SignUp from '../pages/SignUp';
import Home from '../pages/Home';
import ListOfPartners from '../pages/ListOfPartners';
import MyAd from '../pages/MyAd';
import Blank from '../pages/Blank';
import { Auth } from '../helper/CheckAuth';

const PrivateRoute = ({ component: Component, ...rest }) => (
  <Route
    {...rest}
    render={props =>
      Auth.isAuthenticated ? (
        <Component {...props} />
      ) : (
          <Redirect
            to={{
              pathname: "/sign_in",
              state: { from: props.location }
            }}
          />
        )
    }
  />
);

class Routers extends Component {
  render() {
    return (
      <Router>
        <Switch>
          <Route exact path="/" component={Blank} />
          <Route path="/sign_in" component={SignIn} />
          <Route path="/sign_up" component={SignUp} />
          <PrivateRoute path="/home" component={Home} />
          <PrivateRoute path="/search" component={ListOfPartners} />
          <PrivateRoute path="/my_ad" component={MyAd} />
        </Switch>
      </Router>
    )
  }
}

export default Routers;
