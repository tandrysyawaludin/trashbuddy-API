import React, { Component, Fragment } from 'react';
import {
  Collapse,
  Navbar,
  NavbarToggler,
  NavbarBrand,
  Nav,
  NavItem,
  NavLink,
} from 'reactstrap';
import { Link } from 'react-router-dom';

import '../css/NavbarWelcome.css';
class NavbarWelcome extends Component {
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
        <Navbar light expand="md" className="navbar-welcome navbar-default">
          <NavbarBrand className="text-center" href="/">Trashbuddy</NavbarBrand>
          <NavbarToggler className="navbar-toggler-main" onClick={this.toggle} />
          <Collapse isOpen={this.state.isOpen} navbar>
            <Nav className="ml-auto" navbar>
              <NavItem>
                {this.props.atSignUpPage ?
                  <Link to="/" className="nav-link">Sign In</Link> :
                  <Link to="/sign_up" className="nav-link">Sign Up</Link>
                }
              </NavItem>
            </Nav>
          </Collapse>
        </Navbar>
      </Fragment>
    )
  }
}

export default NavbarWelcome