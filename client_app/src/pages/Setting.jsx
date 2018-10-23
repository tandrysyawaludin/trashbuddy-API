import React, { Component, Fragment } from 'react'
import {
  ListGroup, 
  ListGroupItem,
  Row,
  Col,
  Container,
  Button
} from 'reactstrap'
import { Link } from 'react-router-dom'
import CSSModules from 'react-css-modules'
import axios from 'axios'
import * as Cookies from "js-cookie"

import NavbarBottom from '../partials/NavbarBottom'
import NavbarMain from '../partials/NavbarMain'
import styles from '../css/Setting.css'
import { Auth } from '../helper/CheckAuth'

class Setting extends Component {
  handleSignOut = () => {
    let data = {
      is_valid: false
    }

    axios({
      method: 'PUT',
      url: '/signin_log/' + Cookies.get('auth_trashbuddy'),
      data: data
    })
      .then(response => {
        Auth.revoke(() => this.props.history.push('/'))
      })
  }

  render() {
    return (
      <Fragment>
        <NavbarMain prevRoute={this.props.history.goBack} currentRoute={this.props.location.pathname} />
        <div styleName="Setting">
          <Container className="basic-container">
            <Row>
              <Col md={{ size: 6, offset: 3 }}>
                <ListGroup>
                  <ListGroupItem tag="a" href="/">Edit Profile</ListGroupItem>
                  <ListGroupItem tag="a" href="/">About</ListGroupItem>
                </ListGroup>
                <Button 
                  className="logout-button" 
                  outline 
                  block 
                  size="sm" 
                  onMouseDown={this.handleSignOut}
                >
                  Logout
                </Button>
              </Col>
            </Row>
          </Container>
        </div>
        <NavbarBottom currentRoute={this.props.location.pathname} />          
      </Fragment>      
    )
  }
}

export default CSSModules(Setting, styles, { allowMultiple: true })
