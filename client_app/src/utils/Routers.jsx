import React, { Component } from 'react';
import SignIn from '../pages/SignIn'
import SignUp from '../pages/SignUp'
import Home from '../pages/Home'
import ListOfPartners from '../pages/ListOfPartners'
import MyAd from '../pages/MyAd'
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';

class Routers extends Component {
  render() {
    return (
      <Router>
        <Switch>
          <Route exact path="/" component={SignIn} />
          <Route path="/sign_up" component={SignUp} />
          <Route path="/home" component={Home} />
          <Route path="/search" component={ListOfPartners} />
          <Route path="/my_ad" component={MyAd} />
        </Switch>
      </Router>
    )
  }
}

export default Routers;
