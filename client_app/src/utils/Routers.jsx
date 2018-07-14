import React, { Component } from 'react';
import SignIn from '../pages/SignIn'
import SignUp from '../pages/SignUp'
import Home from '../pages/Home'
import ListOfPartner from '../pages/ListOfPartners'
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import 'bootstrap/dist/css/bootstrap.min.css';

class Routers extends Component {
  render() {
    return (
      <Router>
        <Switch>
          <Route exact path="/" component={SignIn} />
          <Route path="/sign_up" component={SignUp} />
          <Route path="/home" component={Home} />
          <Route path={"/search"} component={ListOfPartner} />
        </Switch>
      </Router>
    )
  }
}

export default Routers;
