import React, { Component, Fragment } from 'react';
import PropTypes from 'prop-types';
import {
  Collapse,
  Navbar,
  NavbarBrand,
  Nav,
  NavItem
} from 'reactstrap';
import { Link } from 'react-router-dom';
import MdKeyboardArrowLeft from 'react-icons/lib/md/keyboard-arrow-left';
import CssModules from 'react-css-modules';

import styles from '../css/NavbarMain.css';
import mainLogo from '../img/logo.png'

class NavbarMain extends Component {
  constructor(props) {
    super(props)
  }
  render() {
    return (
      <div styleName="NavbarMain">
        <Navbar fixed="top" light expand="md" className="navbar-default" styleName="navbar-main">
          <MdKeyboardArrowLeft />
          <NavbarBrand href="/" styleName="navbar-brand">
            <img src={mainLogo} /><span>Trashbuddy</span>
          </NavbarBrand>
        </Navbar>
      </div>
    )
  }
}

export default CssModules(NavbarMain, styles, { allowMultiple: true });