import React, { Component } from 'react';
import { BrowserRouter as Router, Route, Switch, Redirect } from 'react-router-dom';

import SignIn from '../pages/SignIn';
import SignUp from '../pages/SignUp';
import Home from '../pages/Home';
import Setting from '../pages/Setting';
import Offer from '../pages/Offer';
import Blank from '../pages/Blank';
import ListOfPartners from '../pages/ListOfPartners';
import MyAd from '../pages/MyAd';

import { Auth } from '../helper/CheckAuth';

const NoMatch = ({ location }) => (
  <div>
    Ups! page is not found
  </div>
);

const PrivateRoute = ({ component: Component, ...rest }) => (
  <Route
    {...rest}
    render={props =>
      Auth.isAuthenticated ? (
        <Component {...props} />
      ) : (
          <Redirect
            to={{
              pathname: "/",
              state: { from: props.location }
            }}
          />
        )
    }
  />
);

const PublicRoute = ({ component: Component, ...rest }) => (
  <Route
    {...rest}
    render={props =>
      Auth.isAuthenticated ? (
        <Redirect
          to={{
            pathname: "/home",
            state: { from: props.location }
          }}
        />
      ) : (
        <Component {...props} />
      )
    }
  />
);

class Routers extends Component {
  render() {
    return (
      <Router>
        <Switch>
          <PublicRoute exact path="/" component={SignIn} />
          <PublicRoute path="/sign_in" component={SignIn} />
          <PublicRoute path="/sign_up" component={SignUp} />
          <PrivateRoute path="/home" component={Home} />
          <PrivateRoute path="/search" component={ListOfPartners} />
          <PrivateRoute path="/my_ad" component={MyAd} />
          <PrivateRoute path="/setting" component={Setting} />
          <PrivateRoute path="/offer" component={Offer} />
          <Route component={NoMatch} />
        </Switch>
      </Router>
    )
  }
}

export default Routers;
