import React, { Component, Fragment } from 'react';
import PropTypes from 'prop-types';
import {
  Collapse,
  Navbar,
  NavbarToggler,
  NavbarBrand,
  Nav,
  NavItem,
  NavLink,
  UncontrolledDropdown,
  DropdownToggle,
  DropdownMenu,
  DropdownItem
} from 'reactstrap';
import { Link } from 'react-router-dom';
import TiHome from 'react-icons/lib/ti/home';
import MdAddCircle from 'react-icons/lib/md/add-circle';
import MdSettings from 'react-icons/lib/md/settings';
import MdAssignment from 'react-icons/lib/md/assignment';
import CssModules from 'react-css-modules';

import styles from '../css/NavbarBottom.css';
class NavbarBottom extends Component {
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
      <Navbar fixed="bottom" light expand="md" styleName="NavbarBottom">
        <Nav>
          <NavItem styleName="nav-item">
            <NavLink href="#"><TiHome /><span>Home</span></NavLink>
          </NavItem>
          <NavItem styleName="nav-item">
            <NavLink href="#"><MdAddCircle /><span>Post</span></NavLink>
          </NavItem>
          <NavItem styleName="nav-item">
            <NavLink href="#"><MdAssignment /><span>Offer</span></NavLink>
          </NavItem>
          <NavItem styleName="nav-item">
            <NavLink href="#"><MdSettings /><span>Account</span></NavLink>
          </NavItem>
        </Nav>
      </Navbar>
    )
  }
}

export default CssModules(NavbarBottom, styles, { allowMultiple: true });
