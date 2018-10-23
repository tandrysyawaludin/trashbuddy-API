import React, { Component, Fragment } from 'react'
import {
  Container,
  Row,
  Col,
  Card,
  CardTitle,
  CardBody,
  CardText,
  CardLink,
  Form,
  FormGroup,
  Label,
  Input,
  Button,
  Alert
} from 'reactstrap'
import { Link } from 'react-router-dom'
import axios from 'axios'
import CSSModules from 'react-css-modules'
import * as Cookies from "js-cookie"
import {
  mapKeys,
  toLower
} from 'lodash'

import NavbarWelcome from '../partials/NavbarWelcome'
import styles from '../css/SignIn.css'
import loader from '../img/loader.svg'
import { Auth } from '../helper/CheckAuth'

class SignIn extends Component {
  state = {
    email: "",
    password: "",
    errorSignIn: false,
    errorSignInMessage: "",
    submitting: false,
  }

  handleInputChange = (event) => {
    const value = event.target.value
    const name = event.target.name

    this.setState({
      [name]: value
    })
  }

  handleSignIn = (event) => {
    event.preventDefault()
    event.stopPropagation()

    console.log('masuk')
    

    this.setState({ submitting: true })

    let data = {
      email: toLower(this.state.email),
      password: this.state.password
    }

    axios({
      method: 'POST',
      url: '/supplier/auth',
      data: data
    })
    .then(response => {
      this.handleAuthSuccess(response)
    })
    .catch(error => {
      this.handleAuthFailed()
    })
    .then(() => {
      this.setState({ submitting: false })
    })
  }

  handleAuthSuccess(response) {
    let data = {}
    if (response.data.success === true) {
      data = {
        user_id: response.data.data.id,
        token: response.data.jwt,
        is_valid: true
      }
      this.handleSaveToken(data)
    }
    else {
      this.setState({ errorSignIn: true, errorSignInMessage: "Email and password do not match" })
      let cookies = Cookies.get()
      mapKeys(cookies, (val, key) => {
        Cookies.remove(key)
      })
    }
  }

  handleAuthFailed() {
    this.setState({ errorSignIn: true, errorSignInMessage: "Sorry, our system is busy now :(" })
    let cookies = Cookies.get()
    mapKeys(cookies, (val, key) => {
      Cookies.remove(key)
    })
  }

  handleSaveToken(data) {
    axios({
      method: 'POST',
      url: '/signin_log',
      data: data
    })
      .then(response => {
        Cookies.set('auth_trashbuddy', data.token, { expires: 1 })
        Auth.authenticate(() => this.props.history.push('/home'))
      })
      .catch(error => {

      })
      .then(() => {
      })
  }

  handleShowAlert = () => {
    this.setState({ errorSignIn: false })
  }

  renderAlert = () => (
    <Alert color="danger" isOpen={this.state.errorSignIn} toggle={this.handleShowAlert}>
      {this.state.errorSignInMessage}
    </Alert>
  )

  renderFormSignin = () => (
    <Form onSubmit={this.handleSignIn}>
      <FormGroup>
        <Label for="exampleEmail">Email</Label>
        <Input type="email" name="email" placeholder="please input valid email" onChange={this.handleInputChange} />
      </FormGroup>
      <FormGroup>
        <Label for="exampleEmail">Password</Label>
        <Input type="password" name="password"
          placeholder="input your password" onChange={this.handleInputChange} />
      </FormGroup>
      <CardText>
        <small className="text-muted">Click Sign In button is accept our <CardLink href="#">Terms and Privacy</CardLink></small>
      </CardText>
      <FormGroup>
        <Button 
          color="main" 
          size="md" 
          block
          type="submit" 
          disabled={this.state.submitting}
        >
          {this.state.submitting ? <img src={loader} /> : "Sign In"}
        </Button>
      </FormGroup>
    </Form>
  )

  renderLinks = () => (
    <CardText>
      <small className="text-muted">
        <Link to="/">About</Link>
        {} . <Link to="/">Forgot Password?</Link>
        {} . <Link to="/sign_up">Sign Up</Link>
      </small>
    </CardText>
  )

  render() {
    return (
      <Fragment>
        <NavbarWelcome />
        <div styleName="SignIn">
          <Container className="singin-form-container">
            <Row>
              <Col md={{ size: 6, offset: 3 }}>
                {this.renderAlert()}
                <Card>
                  <CardBody>
                    <CardTitle className="text-center">Sign iIn</CardTitle>
                    {this.renderFormSignin()}
                    {this.renderLinks()}
                  </CardBody>
                </Card>
              </Col>
            </Row>
          </Container>
        </div>
      </Fragment>
    )
  }
}

export default CSSModules(SignIn, styles, { allowMultiple: true })
