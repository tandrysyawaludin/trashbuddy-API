import React, { Component, Fragment } from 'react';
import {
  Collapse,
  Navbar,
  NavbarBrand
} from 'reactstrap';
import { Link } from 'react-router-dom';

import '../css/NavbarWelcome.css';
import mainLogo from '../img/logo.png'
class NavbarWelcome extends Component {
  render() {
    return (
      <Fragment>
        <Navbar light expand="md" className="navbar-welcome navbar-default">
          <NavbarBrand href="/">
            <img src={mainLogo} /><span>Trashbuddy</span>
          </NavbarBrand>          
        </Navbar>
      </Fragment>
    )
  }
}

export default NavbarWelcome