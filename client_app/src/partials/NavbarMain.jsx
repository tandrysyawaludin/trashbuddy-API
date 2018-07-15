import React, { Component, Fragment } from 'react';
import PropTypes from 'prop-types';
import {
  Collapse,
  Navbar,
  NavbarToggler,
  NavbarBrand,
  Nav,
  NavItem,
  UncontrolledDropdown,
  DropdownToggle,
  DropdownMenu,
  DropdownItem
} from 'reactstrap';
import { Link } from 'react-router-dom';
import MdKeyboardArrowLeft from 'react-icons/lib/md/keyboard-arrow-left';

import '../css/NavbarMain.css';
class NavbarMain extends Component {
  constructor(props) {
    super(props);

    this.toggle = this.toggle.bind(this);
    this.state = {
      isOpen: false
    };
  }
  toggle() {
    this.setState({
      isOpen: !this.state.isOpen
    });
  }
  render() {
    return (
      <Fragment>
        <Navbar fixed="top" light expand="md" className="navbar-main navbar-default">
          <NavbarBrand href="/">
            <MdKeyboardArrowLeft />
            <span>Trashbuddy</span>
          </NavbarBrand>
        </Navbar>
      </Fragment>
    )
  }
}

export default NavbarMain