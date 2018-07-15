import React, { Component, Fragment } from 'react';
import {
  Collapse,
  Navbar,
  NavbarBrand
} from 'reactstrap';
import { Link } from 'react-router-dom';

import '../css/NavbarWelcome.css';
class NavbarWelcome extends Component {
  render() {
    return (
      <Fragment>
        <Navbar light expand="md" className="navbar-welcome navbar-default">
          <NavbarBrand className="text-center" href="/">Trashbuddy</NavbarBrand>
        </Navbar>
      </Fragment>
    )
  }
}

export default NavbarWelcome