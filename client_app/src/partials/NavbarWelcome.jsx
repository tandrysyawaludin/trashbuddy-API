import React, { Component, Fragment } from 'react';
import {
  Collapse,
  Navbar,
  NavbarBrand
} from 'reactstrap';
import { Link } from 'react-router-dom';
import CssModules from 'react-css-modules';

import styles from '../css/NavbarWelcome.css';
import mainLogo from '../img/logo.png'
class NavbarWelcome extends Component {
  render() {
    return (
      <Navbar light expand="md" styleName="NavbarWelcome">
        <NavbarBrand href="/" styleName="navbar-brand">
          <img src={mainLogo} /><span>Trashbuddy</span>
        </NavbarBrand>          
      </Navbar>
    )
  }
}

export default CssModules(NavbarWelcome, styles, { allowMultiple: true });