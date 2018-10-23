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
import {
  map,
  size,
  startCase
} from 'lodash'
import AsyncSelect from 'react-select/lib/Async'

import NavbarWelcome from '../partials/NavbarWelcome'
import loader from '../img/loader.svg'

import '../css/SignUp.css'
class SignUp extends Component {
  state = {
    email: "",
    password: "",
    password_1: "",
    full_name: "",
    phone_number: "",
    area: "",
    address: "",
    errorSignUpMessage: "",
    errorSignUp: false,
    submitting: false,
    optionsArea: []
  }

  componentDidMount() {
    this.getOptionsArea()
  }

  getOptionsArea() {
    axios({
      method: 'GET',
      url: '/areas'
    })
      .then(response => {
        this.setterDataAreas(response)        
      })
      .catch(error => {
        console.log(error)
      })
  }

  setterDataAreas = (response) => {
    const DATA = []
    let area = ''
    map(response.data, (val) => {
      area = startCase(val.province_name) + ", " +
        startCase(val.district_name) + ", " +
        startCase(val.sub_district_name)
      DATA.push({
        "value": val.sub_district_id,
        "label": area
      })
    })
    this.setState({ optionsArea: DATA })   
  }

  handleChangeArea = (area) => {
    this.setState({ area: area.value })
  }

  loadOptionsArea = (input, callback) => {    
    const DATA = this.state.optionsArea    
    if (size(input) > 3) {
      let optionsArea = DATA.filter(i =>
        i.label.toLowerCase().includes(input.toLowerCase())
      )
      setTimeout(() => {
        callback(optionsArea)
      }, 1000)
    }
  }

  handleInputChange = (event) => {
    const value = event.target.value
    const name = event.target.name

    console.log(value, name);
    
    this.setState({
      [name]: value
    }, () => {
      if (name === "password" || name === "password_1") {
        (this.state.password !== this.state.password_1) ?
          this.setState({
            errorSignUpMessage: "Password and Confirmation Password are not match",
            errorSignUp: true
          }) :
          this.setState({ errorSignUp: false })
      }
    })
  }

  handleSubmit = (event) => {
    event.preventDefault()
    event.stopPropagation()

    this.setState({ submitting: true })

    const data = {
      email: this.state.email,
      password: this.state.password,
      name: this.state.full_name,
      phone_number: this.state.phone_number,
      area: this.state.area,
      address: this.state.address
    }

    const headers = {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    }

    axios({
      method: 'POST',
      url: '/supplier',
      data: data
    })
      .then(response => {
        this.props.history.push('/')
      })
      .catch(error => {

      })
      .then(() => {
        this.setState({ submitting: false })
      })
  }

  handleShowAlert = () => {
    this.setState({ errorSignUp: false })
  }

  renderFormSignUp = () => (
    <Form onSubmit={this.handleSubmit}>
      <FormGroup>
        <Label>Email</Label>
        <Input 
          type="email" 
          name="email" 
          placeholder="please input valid email" 
          onChange={this.handleInputChange} 
        />
      </FormGroup>
      <FormGroup>
        <Label>Password</Label>
        <Input 
          type="password" 
          name="password" 
          placeholder="input your password" 
          onChange={this.handleInputChange} 
        />
      </FormGroup>
      <FormGroup>
        <Label>Confirmation Password</Label>
        <Input 
          type="password" 
          name="password_1" 
          placeholder="input your password again" 
          onChange={this.handleInputChange} 
        />
      </FormGroup>
      <FormGroup>
        <Label>Full Name</Label>
        <Input 
          type="text" 
          name="full_name" 
          placeholder="input your full name" 
          onChange={this.handleInputChange} 
        />
      </FormGroup>
      <FormGroup>
        <Label>Area</Label>
        <AsyncSelect
          classNamePrefix="select-input"
          cacheOptions
          loadOptions={this.loadOptionsArea}
          defaultOptions
          loadingMessage={() => "minimal 3 character"}
          noOptionsMessage={() => "area is not found"}
          onChange={this.handleChangeArea}
        />
      </FormGroup>
      <FormGroup>
        <Label>Address</Label>
        <Input type="textarea" name="address" placeholder="input your address" onChange={this.handleInputChange} />
      </FormGroup>
      <FormGroup>
        <Label>Phone Number</Label>
        <Input type="text" name="phone_number" placeholder="input your active phone number" onChange={this.handleInputChange} />
      </FormGroup>
      <CardText>
        <small className="text-muted">Click Sign Up button is accept our <CardLink href="#">Terms and Privacy</CardLink></small>
      </CardText>
      <FormGroup>
        <Button 
          color="main" 
          size="md" 
          block
          disabled={this.state.submitting}
        >
          {this.state.submitting ? <img src={loader} /> : "Sign Up"}
        </Button>
      </FormGroup>
    </Form>
  )

  renderLinks = () => (
    <CardText>
      <small className="text-muted">
        <Link to="/">About</Link>
        {} . <Link to="/">Sign In</Link>
      </small>
    </CardText>
  )

  render() {
    return (
      <Fragment>
        <NavbarWelcome atSignUpPage={true} />
        <Container className="singup-form-container">
          <Row>
            <Col md={{ size: 6, offset: 3 }}>
              <Alert color="danger" isOpen={this.state.errorSignUp} toggle={this.handleShowAlert}>
                {this.state.errorSignUpMessage}
              </Alert>
              <Card>
                <CardBody>
                  <CardTitle className="text-center">Sign Up</CardTitle>
                  {this.renderFormSignUp()}
                  {this.renderLinks()}
                </CardBody>
              </Card>
            </Col>
          </Row>
        </Container>
      </Fragment>
    )
  }
}

export default SignUp