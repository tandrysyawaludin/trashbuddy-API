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

import '../css/NavbarMain.css';
import mainLogo from '../img/logo.png'
class NavbarMain extends Component {
  constructor(props) {
    super(props)
  }
  render() {
    return (
      <Fragment>
        <Navbar fixed="top" light expand="md" className="navbar-main navbar-default">
          <MdKeyboardArrowLeft />
          <NavbarBrand href="/">
            <img src={mainLogo} /><span>Trashbuddy</span>
          </NavbarBrand>
        </Navbar>
      </Fragment>
    )
  }
}

export default NavbarMain