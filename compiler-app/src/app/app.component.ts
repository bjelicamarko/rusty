import { Component } from '@angular/core';
import { AppService } from './app-service.service';
import { CustomReport } from './report';
import { Program } from './program';
import { MatDialog } from '@angular/material/dialog';
import { DialogComponent } from './dialog/dialog.component';
import { Diagnostic } from './diagnostic';
import { ParserType } from './parser-type';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'compiler-app';
  report: CustomReport = {
    diagnostics: [],
    symbol_table: [],
    seconds: 0.0
  };
  program: Program = {
    code: `{ 
  let a = 0; 
  for (j = 0 to 10) { 
    a = a + j; 
  }
  let b = 0;
  if (b == 0) {
    b = 4;
  } else {
    b = 5;
  }
  let c = 0;
  while (c < 5) {
    c = c + 1;
  }
  let d = 0;
  { 
    let x = 3;
    d = x;
  }
}`,
    parser: ParserType.Recursive
  }
  errors: Diagnostic[] = [];
  server_error: boolean = false;

  options = [
    { value: 'Recursive', label: 'Recursive' },
    { value: 'Lr', label: 'Lr' },
    { value: 'Glr', label: 'Glr' }
  ];

  constructor(private appService: AppService, public dialog: MatDialog) { }

  submitProgram(): void {
    this.errors = [];
    this.report = {
      diagnostics: [],
      symbol_table: [],
      seconds: 0.0
    };
    this.server_error = false;
    this.appService.getResult(this.program).subscribe(
      (response) => {
        this.report = response.body as CustomReport;
        console.log('Server response:', this.report);
        this.checkForErrors();
        this.report.symbol_table.sort((a, b) => a.id.localeCompare(b.id));
      },
      (error) => {
        this.server_error = true;
        console.error('Error posting data', error);
      }
    );
  }


  onSelectChange(event: any) {
    this.program.parser = event.target.value;
  }

  checkForErrors(): void {
    this.report.diagnostics.forEach((diagnostic) => {
      if (diagnostic.kind === 'Error') {
        this.errors.push(diagnostic);
      }
    })
  }

  openDialog(): void {
    this.dialog.open(DialogComponent, {
      width: '1200px',
      data: { diagnostics: this.report.diagnostics }
    });
  }
}
