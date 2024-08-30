import { Component } from '@angular/core';
import { AppService } from './app-service.service';
import { CustomReport } from './report';
import { Program } from './program';
import { MatDialog } from '@angular/material/dialog';
import { DialogComponent } from './dialog/dialog.component';
import { Diagnostic } from './diagnostic';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'compiler-app';
  report: CustomReport = {
    diagnostics: [],
    symbol_table: []
  };
  program: Program = {
    code: `{ 
  let res = 0; 
  for (j = 0 to 10) { 
    res = res + j; 
  }
}`
  }
  errors: Diagnostic[] = [];

  constructor(private appService: AppService, public dialog: MatDialog) { }

  submitProgram(): void {
    this.errors = [];
    this.appService.getResult(this.program).subscribe(
      (response) => {
        this.report = response.body as CustomReport;
        console.log('Server response:', this.report);
        this.checkForErrors();
      },
      (error) => {
        console.error('Error posting data', error);
      }
    );
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
